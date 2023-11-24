use auth::ports;
use auth::ports::services::token_service::*;

mod claims;
use claims::*;

pub struct JWTService {
    jwt_ttl: usize,
    public_key: jsonwebtoken::DecodingKey,
    private_key: jsonwebtoken::EncodingKey,
    header: jsonwebtoken::Header,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("")]
    InvalidJWTString,
}
impl From<Error> for ports::Error {
    fn from(err: Error) -> Self {
        ports::Error::InfrastructureError(Box::new(err))
    }
}

struct JWTError(jsonwebtoken::errors::Error);
impl From<JWTError> for ports::Error {
    fn from(wrap: JWTError) -> Self {
        ports::Error::InfrastructureError(Box::new(wrap.0))
    }
}

impl JWTService {
    pub fn es256_from_pem_with_ttl(
        public_pem: &[u8],
        private_pem: &[u8],
        jwt_ttl: usize,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(JWTService {
            jwt_ttl,
            public_key: jsonwebtoken::DecodingKey::from_ec_pem(public_pem)?,
            private_key: jsonwebtoken::EncodingKey::from_ec_pem(private_pem)?,
            header: jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES256),
        })
    }
}

impl TokenService for JWTService {
    fn issue(&self, dto: IssueTokenDTO) -> Result<TokenPairDTO, auth::ports::Error> {
        let now = jsonwebtoken::get_current_timestamp() as usize;
        let claims = Claims {
            user_id: dto.user_id,
            permission_group: dto.permission_group,
            exp: now + self.jwt_ttl,
        };

        let token = jsonwebtoken::encode::<Claims>(&self.header, &claims, &self.private_key)
            .map_err(JWTError)?;
        Ok(TokenPairDTO {
            token,
            refresh_token: generate_refresh_token(),
        })
    }

    fn renew(&self, dto: RenewTokenDTO) -> Result<TokenPairDTO, ports::Error> {
        let mut validation = jsonwebtoken::Validation::new(self.header.alg);
        validation.validate_exp = false;
        let mut old_token =
            jsonwebtoken::decode::<Claims>(&dto.token, &self.public_key, &validation)
                .map_err(JWTError)?;

        let now = jsonwebtoken::get_current_timestamp() as usize;
        old_token.claims.exp = now + self.jwt_ttl;

        let new_token =
            jsonwebtoken::encode::<Claims>(&self.header, &old_token.claims, &self.private_key)
                .map_err(JWTError)?;

        Ok(TokenPairDTO {
            token: new_token,
            refresh_token: generate_refresh_token(),
        })
    }

    fn verify(&self, dto: VerifyTokenDTO) -> Result<bool, ports::Error> {
        let token_parts: Vec<&str> = dto.token.split('.').collect();

        if token_parts.len() != 3 {
            return Err(Error::InvalidJWTString.into());
        }

        let header_and_claims = format!("{}.{}", token_parts[0], token_parts[1]);
        let signature = format!("{}", token_parts[2]);

        Ok(jsonwebtoken::crypto::verify(
            &signature,
            header_and_claims.as_bytes(),
            &self.public_key,
            self.header.alg,
        )
        .map_err(JWTError)?)
    }
}

fn generate_refresh_token() -> String {
    uuid::Uuid::new_v4().to_string()
}
