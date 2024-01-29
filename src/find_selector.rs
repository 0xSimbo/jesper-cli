use keccak_hash::keccak256;

pub fn find_selector(abi: &str) -> String {
    let mut abi_u8_bytes = abi.as_bytes().to_vec();
    //hash it
    keccak256(&mut abi_u8_bytes);
    //take the first 4 bytes
    // Take the first 4 bytes of the hash and convert them to a hex string
    let selector = abi_u8_bytes[0..4]
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    return "0x".to_owned() + &selector;
}
