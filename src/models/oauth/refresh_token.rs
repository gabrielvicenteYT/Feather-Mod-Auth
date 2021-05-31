pub struct RefreshToken {
    pub key: String,
    /// The attached authorization ID, foreigh key for `minos.authorizations`
    pub authorization_id: String,
    /// The time where this refresh token is marked as removed.
    /// This is useful our case as we set the expiral of a refresh token to be 2m after the creation
    /// of a new one.
    pub removal_time: String,
}