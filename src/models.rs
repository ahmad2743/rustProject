#[derive(Debug)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub(crate) complexity: u32,
    // message to sign
    pub(crate) message: String,
}
#[derive(Debug)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub(crate) seed: u64,
    // hashcode found using seed + message
    pub(crate) hashcode: String,
}

