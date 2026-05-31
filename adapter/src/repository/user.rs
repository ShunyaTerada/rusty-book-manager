use kernel::{
    model::{
        id::UserId,
        user::{
            User,
            event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
        },
    },
    repository::user::UserRepository,
};

use crate::database::{ConnectionPool, model::user::UserRow};

use async_trait::async_trait;
use shared::error::{AppError, AppResult};

#[derive(derive_new::new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as!(
            UserRow,
            r#"
                SELECT 
                    u.user_id,
                    u.name,
                    u.email,
                    r.name as role_name,
                    u.created_at,
                    u.updated_at
                FROM users AS u
                INNER JOIN roles AS r USING(role_id)
                WHERE u.user_id = $1
            "#,
            current_user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(User::try_from(r)?)),
            None => Ok(None),
        }
    }
    async fn find_all(&self) -> AppResult<Vec<User>> {
        let row: Vec<UserRow> = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id,
                    u.name,
                    u.email,
                    r.name AS role_name,
                    u.created_at,
                    u.updated_at
                FROM users AS u
                INNER JOIN roles AS r USING()
                ORDER BY created DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.into_iter().map(User::from).collect())
    }
    async fn create(&self, event: CreateUser) -> AppResult<()> {
        todo!()
    }
    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()> {
        todo!()
    }
    async fn update_role(&self, event: UpdateUserRole) -> AppResult<()> {
        todo!()
    }
    async fn delete(&self, event: DeleteUser) -> AppResult<()> {
        todo!()
    }
}
