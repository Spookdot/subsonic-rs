use crate::traits::SubsonicAuthenticationTrait;
use crate::auth::hash_password;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum SubsonicAuthentication {
    LegacyPassword {
        #[serde(rename = "u")]
        username: Box<str>,
        #[serde(rename = "p")]
        password: Box<str>,
    },
    HashedPassword {
        #[serde(rename = "u")]
        username: Box<str>,
        #[serde(rename = "t")]
        hashed_password: Box<str>,
        #[serde(rename = "s")]
        salt: Box<str>,
    },
}

impl SubsonicAuthenticationTrait for SubsonicAuthentication {
    fn legacy_password(username: &str, password: &str) -> Self {
        Self::LegacyPassword { username: username.into(), password: password.into() }
    }
    fn hashed_password(username: &str, password: &str) -> Self {
        let (hashed_password, salt) = hash_password(password);

        Self::HashedPassword { 
            username: username.into(), 
            hashed_password: hashed_password.into(), 
            salt: salt.into() 
        }
    }
}
