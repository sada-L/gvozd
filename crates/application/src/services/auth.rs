use std::sync::Arc;

use domain::{
    models::user::User,
    repository::{token::TokenService, user::UserRepository},
};

use crate::{
    dto::{token::TokenDTO, user::UserDTO},
    errors::AppError,
};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    token_serv: Arc<dyn TokenService>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_serv: Arc<dyn TokenService>) -> Self {
        Self {
            user_repo,
            token_serv,
        }
    }

    pub async fn signup(&self, user_dto: UserDTO) -> Result<TokenDTO, AppError> {
        let user: User = user_dto.try_into()?;
        self.user_repo.create(&user).await?;
        Ok(TokenDTO::from(self.token_serv.generate_token(&user)?))
    }
}
