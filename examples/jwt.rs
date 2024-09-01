use actix_web::cookie::time::OffsetDateTime;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    username: String,
    rule: String,
    exp: u64,
}

fn main() {
    let v4 = Uuid::new_v4();
    let key = v4.as_bytes();
    let my_claims = Claims {
        user_id: "meida".to_owned(),
        username: "b@b.com".to_owned(),
        rule: "admin".to_owned(),
        // exp: (OffsetDateTime::now_utc().unix_timestamp() - 60 * 60 * 5) as u64,
        exp: OffsetDateTime::now_utc().unix_timestamp() as u64, //1723962788,
    };

    println!("{:?}", OffsetDateTime::now_utc().unix_timestamp());
    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);

    println!("{}", token);
}
