use rand::Rng;

pub fn hash_password(password: &str, cost: u32) -> anyhow::Result<String> {
    Ok(bcrypt::hash(password, cost)?)
}

pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(password, hash)?)
}

// get random hash cost
pub fn get_random_cost() -> u32 {
    rand::thread_rng().gen_range(4..=12)
}
