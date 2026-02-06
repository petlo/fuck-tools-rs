use argon2::password_hash::{Error, phc};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwdUser {
    pub id: i64,
    pub username: String,
    pub email: String,
}

impl PwdUser {
    pub fn new(id: i64, username: String, email: String) -> PwdUser {
        Self { id, username, email }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String, // 用户ID
    pub username: String,
    pub email: String,
    pub exp: i64,            // 过期时间
    pub iat: i64,            // 签发时间
    pub jti: String,         // JWT ID
    pub scopes: Vec<String>, // 权限范围
}

pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash_password(password: &str) -> Result<String, Error> {
        let argon2 = Argon2::default();
        match argon2.hash_password(password.as_bytes()) {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(err) => Err(err),
        }
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, phc::Error> {
        let argon2 = Argon2::default();
        match PasswordHash::new(hash) {
            Ok(password_hash) => {
                let input_pwd = password.as_bytes();
                Ok(argon2.verify_password(input_pwd, &password_hash).is_ok())
            }
            Err(err) => Err(err),
        }
    }

    pub fn generate_token(
        secret: &str,
        expiry_hours: i64,
        pwd_user: &PwdUser,
        scopes: Vec<String>,
    ) -> Result<String, anyhow::Error> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(expiry_hours);

        let claims = JWTClaims {
            sub: pwd_user.id.to_string(),
            username: pwd_user.username.clone(),
            email: pwd_user.email.clone(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
            scopes,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }

    pub fn verify_token(secret: &str, token: &str) -> Result<JWTClaims, anyhow::Error> {
        let token_data = decode::<JWTClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub fn refresh_token(
        secret: &str,
        expiry_hours: i64,
        token: &str,
    ) -> Result<String, anyhow::Error> {
        let claims = PasswordUtils::verify_token(secret, token)?;
        let now = Utc::now();
        let expires_at = now + Duration::hours(expiry_hours);

        let new_claims = JWTClaims {
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
            ..claims
        };

        Ok(encode(
            &Header::default(),
            &new_claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }
}
