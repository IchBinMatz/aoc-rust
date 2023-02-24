use md5;


pub fn mine_advent_coin_hash(secret: &str) -> u32 {
    let digest = md5::compute(secret);
    let mut message: String;
    u128::from_be_bytes(digest.0);
    let mut number:u32 = 0;
    loop {
        message = secret.to_owned();
        message.push_str(&number.to_string());
        let hash = u128::from_be_bytes(md5::compute(message).0);
        if hash < 0x08000000000000000000000000000000 {
            break;
        }
    }
    number
}

#[test]
fn examples() {
    assert_eq!(mine_advent_coin_hash("abcdef"), 609043);
    assert_eq!(mine_advent_coin_hash("pqrstuv"), 1048970);
}

