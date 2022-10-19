use crate::managers::token::Tokeniser;
use crate::managers::um::UserManager;
use crate::models::auth::{UserLoginRequest, UserLoginResponse};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub struct AuthEndpoint
{
    um: UserManager,
    tokeniser: Tokeniser,
}

impl AuthEndpoint {
    pub fn new() -> Self {
        Self {
            um: UserManager::new(),
            tokeniser: Tokeniser::new(),
        }
    }

	pub fn auth(&mut self, user: UserLoginRequest) -> Option<UserLoginResponse> {
		match self.um.find_by_username(&user.username) {
            Some(usr) => {
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());

                if usr.password == sha.result_str() {
                    Some(UserLoginResponse {
                        token: self.tokeniser.generate_tokens(usr, 2),
                    })
                } else {
                    None
                }
            }
            None => None,
        }
	}

}