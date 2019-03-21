extern crate jsonwebtoken as jwt;
extern crate serde;

use chrono::{DateTime, Duration, Utc};
use chrono::serde::ts_seconds;
use jwt::{encode, Header, Algorithm};
use serde::Serialize;

type UtcDateTime = DateTime<Utc>;
type JWTResult = jwt::errors::Result<String>;

// const DEFAULT_DURATION: Duration = Duration::hours(12);

#[derive(Serialize)]
struct Claims {
    iss: String,
    #[serde(with = "ts_seconds")]
    iat: UtcDateTime,
    #[serde(with = "ts_seconds")]
    exp: UtcDateTime
}

pub struct ClaimsBuilder {
    team_id: String,
    exp_duration: Option<Duration>
}

impl ClaimsBuilder {
    pub fn new(team_id: String) -> Self {
        ClaimsBuilder {
            team_id,
            exp_duration: None
        }
    }

    pub fn exp_duration(mut self, duration: Duration) -> Self {
        self.exp_duration = Some(duration);
        self
    }

    fn done(self) -> Claims {
        let now = Utc::now();
        Claims {
            iss: self.team_id,
            iat: now,
            exp: now + self.exp_duration.unwrap_or(Duration::hours(12))
        }
    }
}

pub fn make_token(team_id: &str, key_id: &str, key: &str) -> JWTResult {
    let header = Header {
        kid: Some(key_id.to_owned()),
        alg: Algorithm::HS256, // TODO: change this to ES256
        ..Default::default()
    };
    let claims = ClaimsBuilder::new(team_id.to_owned())
        .exp_duration(Duration::days(1))
        .done();
    encode(&header, &claims, key.as_ref())
}
