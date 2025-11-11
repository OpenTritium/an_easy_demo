use crate::{
    model::{Password, User, UserId, Username},
    repo::{
        create_user, delete_user, get_all_users, get_user_by_id, get_user_by_username,
        update_user_password,
    },
};
use axum::{
    Json, Router,
    extract::Path,
    routing::{get, put},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response<T: Serialize> {
    was_success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    msg: String,
}

impl<T: Serialize> Response<T> {
    #[inline]
    fn with_error(msg: String) -> Self {
        Response {
            was_success: false,
            data: None,
            msg,
        }
    }
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: Username,
    password: Password,
}

#[derive(Deserialize)]
struct UpdatePasswordRequest {
    old_password: Password,
    new_password: Password,
}

#[derive(Deserialize)]
struct ConfirmPasswordRequest {
    sample: Password,
}

/// 注册用户
#[axum::debug_handler]
async fn handle_create_user(Json(payload): Json<CreateUserRequest>) -> Json<Response<User>> {
    let resp = create_user(&payload.username, &payload.password)
        .await
        .map(|user| Response {
            was_success: true,
            data: Some(user),
            msg: "User created successfully".to_string(),
        })
        .unwrap_or_else(|err| Response::with_error(format!("Failed to create user: {err}")));
    Json(resp)
}

/// 通过 id 获取用户
#[axum::debug_handler]
async fn handle_get_user(Path(id): Path<UserId>) -> Json<Response<User>> {
    let resp = match get_user_by_id(UserId::from(id)).await {
        Ok(Some(user)) => Response {
            was_success: true,
            data: Some(user),
            msg: "User retrieved successfully".to_string(),
        },
        Ok(None) => Response::with_error("User was not found".to_string()),
        Err(err) => Response::with_error(format!("Failed to retrieve user: {err}")),
    };
    Json(resp)
}

/// 通过用户名获取用户
#[axum::debug_handler]
async fn handle_get_user_by_username(Path(username): Path<String>) -> Json<Response<User>> {
    let resp = match get_user_by_username(&username).await {
        Ok(Some(user)) => Response {
            was_success: true,
            data: Some(user),
            msg: "User retrieved successfully".to_string(),
        },
        Ok(None) => Response::with_error("User was not found".to_string()),
        Err(err) => Response::with_error(format!("Failed to retrieve user: {err}")),
    };
    Json(resp)
}

// 获取所有用户
#[axum::debug_handler]
async fn handle_get_all_users() -> Json<Response<Vec<User>>> {
    let resp = get_all_users()
        .await
        .map(|users| Response {
            was_success: true,
            data: Some(users),
            msg: "Users retrieved successfully".to_string(),
        })
        .unwrap_or_else(|err| {
            Response::with_error(format!("Failed to retrieve users with err: {err}"))
        });
    Json(resp)
}

/// 校验密码
#[axum::debug_handler]
async fn handle_confirm_password(
    Path(id): Path<UserId>,
    Json(payload): Json<ConfirmPasswordRequest>,
) -> Json<Response<()>> {
    let cur_password = match get_user_by_id(id).await {
        Ok(Some(user)) => user.password,
        Ok(None) => return Json(Response::with_error("User was not found".to_string())),
        Err(_) => return Json(Response::with_error("Failed to retrieve user".to_string())),
    };
    let resp = if payload.sample == cur_password {
        Response {
            was_success: true,
            data: None,
            msg: "Password comfirmed successfully".to_string(),
        }
    } else {
        Response::with_error("Password comfirmed failed".to_string())
    };
    Json(resp)
}

/// 更新用户密码
#[axum::debug_handler]
async fn handle_update_user_password(
    Path(id): Path<UserId>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Json<Response<()>> {
    let sqlx_err =
        |err: sqlx::Error| Response::with_error(format!("Failed to update user password: {err}"));
    let user_not_found = Response::with_error("User was not found".to_string());
    // 先校验一下旧密码
    let cur_passwd = match get_user_by_id(id).await {
        Ok(Some(user)) => user.password,
        Ok(None) => return Json(user_not_found),
        Err(err) => return Json(sqlx_err(err)),
    };
    if cur_passwd != payload.old_password {
        return Json(Response::with_error(
            "Old password is incorrect".to_string(),
        ));
    }
    let resp = match update_user_password(id, &payload.new_password).await {
        Ok(0) => Response::with_error("You can't update password for a not exist user".to_string()),
        Ok(_) => Response {
            was_success: true,
            data: None,
            msg: "User password updated successfully".to_string(),
        },
        Err(err) => sqlx_err(err),
    };
    Json(resp)
}

/// 删除用户
#[axum::debug_handler]
async fn handle_delete_user(Path(id): Path<UserId>) -> Json<Response<()>> {
    let resp = match delete_user(id).await {
        Ok(0) => Response::with_error("User was not found".to_string()),
        Ok(_) => Response {
            was_success: true,
            data: None,
            msg: "User deleted successfully".to_string(),
        },
        Err(err) => Response::with_error(format!("Failed to delete user: {err}")),
    };
    Json(resp)
}

pub fn api_routes() -> Router {
    Router::new()
        .route("/users", get(handle_get_all_users).post(handle_create_user))
        .route(
            "/users/{id}",
            get(handle_get_user).delete(handle_delete_user),
        )
        .route(
            "/users/username/{username}",
            get(handle_get_user_by_username),
        )
        .route(
            "/users/{id}/password",
            put(handle_update_user_password).post(handle_confirm_password),
        )
}
