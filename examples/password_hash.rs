use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

fn main() {
    let password = "123456".as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    // 123456
    // $argon2id$v=19$m=19456,t=2,p=1$9lA5D4TBhTtA27jvoyzNtw$8JII76h1N2i2iS3KYEOM3Em1pWcjXWZzn69NRT8n61k
    let parsed_hash = PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$CMm6eGkzPhrB46S2Vu5Big$44TzewAU4/5B1rPXTUMTJ58dF/G0SP2nEQH2Q7+4a04").unwrap();

    println!("Password hash: {}", password_hash);

    println!(
        "Is valid: {}",
        argon2.verify_password(password, &parsed_hash).is_ok()
    );
}
