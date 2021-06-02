use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;
// Custom base 55 (without near characters: "0oO;IilL")
lazy_static! {
    static ref DICTIONARY: Vec<char> = "abcdefghjkmnpqrstuvwxyzABCDEFGHJKMNPQRSTUVWXYZ123456789"
        .chars()
        .collect();
    static ref RANDOM: Hc128Rng = Hc128Rng::from_entropy();
}
/// Creates a random string of defined length.
/// The resulting string is composed of the following characters:
///
/// All alphanumeric characters (case sensitive) without the following: "0oOIiLl"
/// They are removed to improve readability without impacting much the entropy of the resulting functions.
///
/// Only a few lengths should be used for the sake of uniformity:
/// - 6 characters (503284375 possibilities) for **non sensitive** url displayed elements
/// - 12 characters (6*10^34 possibilities) for non displayed and sensitive elements
///
/// **Secrets SHOULD NOT be generated with this function**
///
/// The [generate_secret] functions uses ring's CSPRNG system, that is saver and more audited.
pub fn generate_id(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(Uniform::new(0, 55))
        .take(len)
        .map(|e| DICTIONARY.get(e).unwrap())
        .collect()
}
/// Creates a random string of defined length.
/// The numbers are generated using the Hc128 CSPRNG system, and is endorsed by the eSTREAM project.
/// The resulting string is composed of the following characters:
/// All alphanumeric characters (case sensitive) without the following: "0oOIiLl"
/// They are removed to improve readability without impacting much the entropy of the resulting functions.
/// Only a few lengths should be used for the sake of uniformity:
/// - 24 characters (10^41 possibilities) for client secrets & refresh tokens
/// - 64 characters (10^111 possibilities) for machine-to-machine secrets
pub fn generate_secret(len: usize) -> String {
    RANDOM
        .clone()
        .sample_iter(Uniform::new(0, 55))
        .take(len)
        .map(|e| DICTIONARY.get(e).unwrap())
        .collect()
}
