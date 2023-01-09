use ring::{digest, hmac, rand};
fn main() {
    // create a range
    let rng = rand::SystemRandom::new();
    // create an array of key_value
    let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng).unwrap().expose();
    // fill the key_value array with random numbers
    // create a key
    let key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
    // message
    let message = "Columbo is on to him";
    // sign the message
    let signature = hmac::sign(&key, message.as_bytes());
    // verify the signature
    hmac::verify(&key, message.as_bytes(), signature.as_ref()).unwrap();
}
