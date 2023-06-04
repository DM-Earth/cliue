use surf::{http::convert::json, StatusCode};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum TokenRequestor {
    /// Request method used by seiue frontend.
    Cookie {
        /// Have not discovered how to get automatically.
        /// Can be found in browser devtools.
        client_id: String,
        cookie: String,
    },
    /// The open API method provided by seiue.
    /// Ask seiue staffs for these two fields.
    OpenApi {
        client_id: String,
        client_secret: String,
    },
}

impl TokenRequestor {
    pub async fn request(&self) -> surf::Result<TokenData> {
        Ok(match self {
            TokenRequestor::Cookie { client_id, cookie } => {
                surf::post("https://passport.seiue.com/authorize")
                    .body_string(format!("client_id={}&response_type=token", client_id))
                    .header("Cookie", cookie)
                    .recv_json::<CookieAuthResponseBody>()
                    .await?
                    .into()
            }
            TokenRequestor::OpenApi {
                client_id,
                client_secret,
            } => surf::post("https://open.seiue.com/api/v3/oauth/tokens")
                .body_json(&json!({
                    "grant_type": "client_credentials",
                    "client_id": client_id,
                    "client_secret": client_secret,
                }))?
                .recv_json::<OpenApiAuthResponseBody>()
                .await?
                .into(),
        })
    }
}

/// The token data for auth.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenData {
    expire_time: chrono::DateTime<chrono::Utc>,
    pub token: String,
    pub token_type: String,
}

impl TokenData {
    pub fn is_expired(&self) -> bool {
        self.expire_time <= chrono::Utc::now()
    }
}

#[derive(serde::Deserialize)]
struct CookieAuthResponseBody {
    token_type: String,
    expires_in: String,
    access_token: String,
    #[serde(rename = "active_reflection_id")]
    _active_reflection_id: String,
}

impl Into<TokenData> for CookieAuthResponseBody {
    fn into(self) -> TokenData {
        TokenData {
            token_type: self.token_type,
            expire_time: chrono::Utc::now()
                + chrono::Duration::seconds(self.expires_in.parse().unwrap_or_default()),
            token: self.access_token,
        }
    }
}

#[derive(serde::Deserialize)]
struct OpenApiAuthResponseBody {
    token_type: String,
    expires_in: u32,
    access_token: String,
}

impl Into<TokenData> for OpenApiAuthResponseBody {
    fn into(self) -> TokenData {
        TokenData {
            token_type: self.token_type,
            expire_time: chrono::Utc::now() + chrono::Duration::seconds(self.expires_in as i64),
            token: self.access_token,
        }
    }
}

/// Request cookies from password, which could be used in [`AuthTokenRequestor`].
pub struct CookieRequestor {
    pub school_id: u32,
    /// It called `email` but actually it don't need to be an email address.
    pub email: String,
    pub password: String,
}

impl CookieRequestor {
    /// Return cookies.
    pub async fn request(&self) -> surf::Result<String> {
        surf::post(format!(
            "https://passport.seiue.com/login?force=1&school_id={}&type=account",
            self.school_id
        ))
        .body_string(format!(
            "email={}&password={}&school_id={}&submit=Submit+Query",
            self.email, self.password, self.school_id
        ))
        .await?
        .header("set-cookie")
        .map_or_else(
            || {
                Err(surf::Error::from_str(
                    StatusCode::NoContent,
                    "set-cookie response header not found",
                ))
            },
            |e| {
                let mut string = String::new();
                for ee in e.iter() {
                    string.push_str(ee.as_str());
                    string.push_str("; ");
                }
                string.pop();
                string.pop();
                Ok(string)
            },
        )
    }
}
