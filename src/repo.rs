use crate::{
    db::global_db,
    model::{User, UserId},
};
use std::ops::Deref;

pub async fn create_user(username: &str, passwd: &str) -> sqlx::Result<User> {
    let db = global_db().await;
    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING id, username, password",
        UserId::new_v4(), // 让应用层数据库生成 UserId 得了，就不用管 pg 有没有缺模块了
        username,
        passwd
    )
    .fetch_one(db.deref())
    .await?;
    Ok(new_user)
}

pub async fn get_user_by_id(id: UserId) -> sqlx::Result<Option<User>> {
    let db = global_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password FROM users WHERE id = $1",
        id
    )
    .fetch_optional(db.deref())
    .await?;
    Ok(user)
}

pub async fn get_user_by_username(username: &str) -> sqlx::Result<Option<User>> {
    let db = global_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password FROM users WHERE username = $1",
        username
    )
    .fetch_optional(db.deref())
    .await?;
    Ok(user)
}

pub async fn get_all_users() -> sqlx::Result<Vec<User>> {
    let db = global_db().await;
    let users = sqlx::query_as!(User, "SELECT id, username, password FROM users")
        .fetch_all(db.deref())
        .await?;
    Ok(users)
}

pub async fn update_user_password(id: UserId, new_passwd: &str) -> sqlx::Result<u64> {
    let db = global_db().await;
    let rows_affected = sqlx::query!(
        "UPDATE users SET password = $1 WHERE id = $2",
        new_passwd,
        id
    )
    .execute(db.deref())
    .await?
    .rows_affected();
    Ok(rows_affected)
}

pub async fn delete_user(id: UserId) -> sqlx::Result<u64> {
    let db = global_db().await;
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db.deref())
        .await?
        .rows_affected();
    Ok(rows_affected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_and_get_user(pool: PgPool) {
        let tx = pool.begin().await.unwrap();
        let username = "test_user_1";
        let password = "password123";

        let new_user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING id, username, password",
            UserId::new_v4(),
            username,
            password
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to create user");

        assert_eq!(new_user.username, username);

        let fetched_user = sqlx::query_as!(
            User,
            "SELECT id, username, password FROM users WHERE id = $1",
            new_user.id
        )
        .fetch_optional(&pool)
        .await
        .expect("Failed to fetch user")
        .expect("User should exist");

        assert_eq!(fetched_user.id, new_user.id);
        tx.rollback().await.unwrap();
    }

    #[sqlx::test]
    async fn test_get_all_users(pool: PgPool) {
        let tx = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO users (id, username, password) VALUES ($1, $2, $3), ($4, $5, $6)")
            .bind(UserId::new_v4())
            .bind("user_a")
            .bind("pass_a")
            .bind(UserId::new_v4())
            .bind("user_b")
            .bind("pass_b")
            .execute(&pool)
            .await
            .unwrap();

        let users = sqlx::query_as!(User, "SELECT id, username, password FROM users")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(users.len(), 2);
        tx.rollback().await.unwrap();
    }

    #[sqlx::test]
    async fn test_update_user_password(pool: PgPool) {
        let tx = pool.begin().await.unwrap();
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING *",
            UserId::new_v4(),
            "user_to_update",
            "old_password"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let new_password = "new_strong_password";
        let rows_affected = sqlx::query!(
            "UPDATE users SET password = $1 WHERE id = $2",
            new_password,
            user.id
        )
        .execute(&pool)
        .await
        .unwrap()
        .rows_affected();

        assert_eq!(rows_affected, 1);
        tx.rollback().await.unwrap()
    }

    #[sqlx::test]
    async fn test_delete_user(pool: PgPool) {
        let tx = pool.begin().await.unwrap();
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING *",
            UserId::new_v4(),
            "user_to_delete",
            "some_password"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let was_deleted = sqlx::query!("DELETE FROM users WHERE id = $1", user.id)
            .execute(&pool)
            .await
            .unwrap()
            .rows_affected()
            > 0;

        assert!(was_deleted);

        let maybe_user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user.id)
            .fetch_optional(&pool)
            .await
            .unwrap();

        assert!(maybe_user.is_none());
        tx.rollback().await.unwrap();
    }
}
