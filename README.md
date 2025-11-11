# an_easy_demo

## API 接口

- `POST /api/v1/users` - 创建新用户
- `GET /api/v1/users` - 获取所有用户列表
- `GET /api/v1/users/{id}` - 根据 ID 获取用户
- `GET /api/v1/users/username/{username}` - 根据用户名获取用户
- `DELETE /api/v1/users/{id}` - 根据 ID 删除用户
- `PUT /api/v1/users/{id}/password` - 更新用户密码
- `POST /api/v1/users/{id}/password` - 验证用户密码

## 数据表

使用 PostgreSQL 数据库，包含一个 users 表：

```sql
CREATE TABLE public.users (
    id uuid NOT NULL,
    username character varying(32) NOT NULL,
    password character varying(255) NOT NULL
);
```

## 环境要求

- Rust 1.78+
- PostgreSQL 数据库

## 配置

项目使用 `.env` 文件进行配置，需要设置数据库连接：

> 需要注意的是，为了方便偷懒就测试和运行都连接的同一个数据库，生产环境不建议这么做。

```env
DATABASE_URL=postgresql://user:password@localhost:port/database_name
```

## 运行项目

```bash
# 克隆项目
git clone <repository-url>

# 进入项目目录
cd a_easy_demo

# 设置环境变量
echo "DATABASE_URL=postgresql://user:password@localhost/database_name" > .env

# 运行数据库迁移（需要预先安装 sqlx-cli）
sqlx migrate run

# 启动项目
cargo run
```

服务将在 `127.0.0.1:7878` 上运行。

此外，你可以安装 REST Client Vscode 扩展，并使用项目根目录下的 `api-tests.http` 文件进行测试。
