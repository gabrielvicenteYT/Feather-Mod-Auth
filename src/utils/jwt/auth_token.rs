use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    /// This is required to be the url of the service that issued the this token
    /// Including scheme (it **has** to be https), host, and optionally port and path.
    pub iss: String,
    /// This is a required field mapping each user to a locally unique and never reassigned
    /// identifier.
    ///
    /// In other words, it's the id of the user.
    pub sub: String,
    /// The audience this token is attributed to.
    /// This contains a list of arbitrary strings or URLs to resources you are allowed to access
    /// using this token.
    pub aud: Vec<String>,
    /// The Authorized party.
    /// This has to be the `client_id` of the authentication request.
    pub azp: String,
    /// This is a required field that contains the date where the ID token is no longer valid.
    ///
    /// The recommended expiration time for this value is **24h**, as described in Auth0
    /// recommendation document.
    pub exp: usize,
    /// This is a required field that describes the time where the token was issued form this
    /// server.
    pub iat: usize,
    /// This is a space separated list of scopes that were requested by the client, and accepted by
    /// the user.
    pub scope: String,
}
