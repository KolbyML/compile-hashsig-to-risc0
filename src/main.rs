use hashsig::{
    MESSAGE_LENGTH,
    signature::{SignatureScheme, SigningError},
};

pub type HashSigScheme = hashsig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
type HashSigSignature = <HashSigScheme as SignatureScheme>::Signature;
pub type HashSigPrivateKey = <HashSigScheme as SignatureScheme>::SecretKey;

pub fn sign(
    private_key: HashSigPrivateKey,
    message: &[u8; MESSAGE_LENGTH],
    epoch: u32,
) -> Result<HashSigSignature, SigningError> {
    Ok(<HashSigScheme as SignatureScheme>::sign(
        &private_key,
        epoch,
        message,
    )?)
}

fn main() {
    println!("Hello, world!");
}
