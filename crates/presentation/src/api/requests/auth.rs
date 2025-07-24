use application::dto::user::UserDTO;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthSignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<AuthSignupRequest> for UserDTO {
    fn from(val: AuthSignupRequest) -> Self {
        UserDTO {
            username: val.username,
            email: val.email,
            password: val.password,
        }
    }
}
