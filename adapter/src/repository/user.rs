use kernel::{
    model::{
        id::UserId,
        role::Role,
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
        let users = sqlx::query_as!(
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
                INNER JOIN roles AS r USING(role_id)
                ORDER BY u.created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .filter_map(|r| User::try_from(r).ok())
        .collect();

        Ok(users)
    }
    async fn create(&self, event: CreateUser) -> AppResult<User> {
        let user_id = UserId::new();
        let hashed_password = hash_password(&event.password)?;
        let role = Role::User;

        let res = sqlx::query!(
            r#"
                INSERT INTO users(name, email, password_hash, role_id)
                SELECT $1, $2, $3, $4, role_id FROM roles WHERE name = $5;
            "#,
            event.name,
            event.email,
            hashed_password,
            role.as_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No user has been created".into(),
            ));
        }

        Ok(User {
            id: user_id,
            name: event.name,
            email: event.email,
            role,
        })
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
