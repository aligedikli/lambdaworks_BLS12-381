use blst::min_pk::SecretKey;
use rand::RngCore;

fn main() {
    // Generate a random 32-byte secret key (for testing purposes only)
    let mut rng = rand::thread_rng();
    let mut sk_bytes = [0u8; 32];
    rng.fill_bytes(&mut sk_bytes);

    // Create a secret key from the bytes
    let secret_key = SecretKey::from_bytes(&sk_bytes).expect("Invalid secret key");

    // Generate the public key
    let public_key = secret_key.sk_to_pk();

    // Convert public key to bytes and then to hex for display
    let pk_bytes = public_key.compress();
    let pk_hex = hex::encode(pk_bytes);

    println!("Public Key: {}", pk_hex);
}
