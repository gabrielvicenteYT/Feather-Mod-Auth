use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdClaims {
    /// This is the `iss` field of the OIDC ID Token claims.
    ///
    /// This is required to be the url of the service that issued the this token
    /// Including scheme (it **has** to be https), host, and optionally port and path.
    pub iss: String,
    /// This is the `sub` field of the OIDC ID Token claims.
    ///
    /// This is a required field mapping each user to a locally unique and never reassigned
    /// identifier.
    ///
    /// In other words, it's the id of the user.
    pub sub: String,
    /// This is the `aud` field of the OIDC ID Token claims.
    ///
    /// This is a required field that has to contain the `client_id` of the requesting
    /// party.
    ///
    /// In addition to the allowed services, for internal tokens.
    pub aud: String,
    /// This is the `exp` field of the OIDC ID Token claims.
    ///
    /// This is a required field that contains the date where the ID token is no longer valid.
    ///
    /// The recommended expiration time for this value is **24h**, as described in Auth0
    /// recommendation document.
    pub exp: usize,
    /// This is the `iat` field of the OIDC ID Token claims.
    ///
    /// This is a required field that describes the time where the token was issued form this server.
    pub iat: usize,
    /// This is the `auth_time` field of the OIDC ID Token claims.
    /// The time where the user last logged in in the app
    ///
    /// This is useful in cases where you want to ensure that the user has confirmed their identity
    /// before touching any sensible data.
    ///
    /// This field is REQUIRED if a max_age request is made or when `auth_time` is requested as an
    /// essential claim.
    pub auth_time: Option<usize>,
    /// This is the `nonce` field of the OIDC ID Token claims.
    ///
    /// A string value used to associate a client session with an ID token.
    ///
    /// This is set to the nonce parameter of the Authentication request
    pub nonce: Option<String>,
    /// This is the `amr` field of the OIDC ID Token claims.
    ///
    /// This is an OPTIONAL field, but it should identify what authentication systems were used to
    /// assert that the user is, in fact, the user.
    ///
    /// If we cannot provide a level of guarantee this field should be set to "0", for example when
    /// logging the user in with a long term cookie.
    ///
    /// In our specific case, we do provide three values other than "0":
    /// - `self-trusted`: User logged in only using their username & password, and has either 2FA
    /// disabled or there was no scope requesting for it.
    /// - `self-mf`: User logged in using their username & password, and then confirmed their
    /// identity using 2FA, or has already trusted the computer.
    /// - `third-party`: Use logged in using a third-party authentication system.
    pub amr: Option<String>,
}
