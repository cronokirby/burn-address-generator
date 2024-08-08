use std::env;

use sha2::Digest;

fn hash_to_element(x: &str) -> decaf377::Element {
    let hash_bytes: [u8; 64] = sha2::Sha512::digest(x.as_bytes()).into();
    decaf377::Element::hash_to_curve(
        &decaf377::Fq::from_le_bytes_mod_order(&hash_bytes[..32]),
        &decaf377::Fq::from_le_bytes_mod_order(&hash_bytes[32..]),
    )
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("expected seed string"));
    }
    let addr = penumbra_keys::Address::from_components(
        [0u8; 16].as_slice().try_into().unwrap(),
        decaf377::Encoding::from(hash_to_element(args[1].as_str()))
            .0
            .as_slice()
            .try_into()
            .unwrap(),
        [0u8; 32].as_slice().try_into().unwrap(),
    )
    .expect("creating address should work");
    println!("{}", addr);
    Ok(())
}
