use md5;
pub fn mine_advent_coin_hash(secret: &str) -> u32 {
    let mut message: String;
    let mut number:u32 = 0;
    loop {
        message = secret.to_owned();
        message.push_str(&number.to_string());
        let hash_0 = md5::compute(&message);
        let hash = u128::from_be_bytes(hash_0.0);
        if hash < 0x00000100000000000000000000000000 {
            println!("{} produces {:?}", &message, hash_0);
            break;
        }
        number = number + 1
    }
    number
}
