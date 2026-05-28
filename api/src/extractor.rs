use axum::{RequestPartsExt, async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use kernel::model::{auth::AccessToken, id::UserId, role::Role, user::User};
use shared::error::AppError;

use registry::AppRegistry;

//リクエストの前処理を実行後、handlerに渡す構造体を定義
pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }

    pub fn id_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;

    //handlerメソッドの引数にAuthorizedUserを追加したときはこのメソッドが呼ばれる
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        //HTTPヘッダからのアクセストークンを取り出す
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError)?;

        let access_token = AccessToken(bearer.token().to_string());

        //アクセストークンが紐づくユーザーIDを抽出する。
        let user_id: UserId = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        //ユーザーIDでデータベースからユーザーのレコードを引く。
        let user: User = registry
            .user_repository()
            .find_current_user(user_id)
            .await
            .ok_or(AppError::UnauthorizedError)?;

        Ok(Self { access_token, user })
    }
}
