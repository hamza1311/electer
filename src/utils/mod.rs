use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
