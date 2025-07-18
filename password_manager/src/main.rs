

use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::{decode, encode};
use rand::Rng;








const KEY: &[u8] = b"an example very very secret key.";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Password {
    username: String,
    encrypted_password: String,
}

#[derive(Debug, Deserialize)]
struct MasterPassword {
    master_password: String,
}

type Db = Arc<Mutex<HashMap<String, Password>>>;

fn encrypt_password(plain_text: &str) -> String {
    let key = GenericArray::from_slice(KEY);
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plain_text.as_bytes()).expect("encryption failure!");

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    encode(&result)
}

fn decrypt_password(cipher_text: &str) -> String {
    let key = GenericArray::from_slice(KEY);
    let cipher = Aes256Gcm::new(key);

    let decoded = decode(cipher_text).expect("decoding failure!");
    let (nonce, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("decryption failure!");
    String::from_utf8(plaintext).expect("utf-8 conversion failure!")
}

async fn add_password(new_password: Password, db: Db) -> Result<impl Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();
    let encrypted_password = encrypt_password(&new_password.encrypted_password);
    db.insert(
        new_password.username.clone(),
        Password {
            username: new_password.username,
            encrypted_password,
        },
    );
    save_db(&db).expect("Failed to save database");
    Ok(warp::reply::with_status("Password added", warp::http::StatusCode::CREATED))
}

async fn get_passwords(master_password: MasterPassword, db: Db) -> Result<impl Reply, warp::Rejection> {
    if master_password.master_password != "bb" {
        return Err(warp::reject::not_found());
    }

    let db = db.lock().unwrap();
    let passwords: Vec<Password> = db.values().map(|p| {
        Password {
            username: p.username.clone(),
            encrypted_password: decrypt_password(&p.encrypted_password),
        }
    }).collect();

    let passwords_json = warp::reply::json(&passwords);
    let response = passwords_json.into_response();

    Ok(response)
}

fn save_db(db: &HashMap<String, Password>) -> std::io::Result<()> {
    let file = File::create("passwords.json")?;
    serde_json::to_writer(file, db)?;
    Ok(())
}

fn load_db() -> std::io::Result<HashMap<String, Password>> {
    let file = File::open("passwords.json")?;
    let db: HashMap<String, Password> = serde_json::from_reader(file)?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(load_db().unwrap_or_else(|_| HashMap::new())));

    let add_password_route = warp::path("add_password")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(add_password);

    let get_passwords_route = warp::path("passwords")
        .and(warp::get())
        .and(warp::query::<MasterPassword>())
        .and(with_db(db.clone()))
        .and_then(get_passwords);

    warp::serve(add_password_route.or(get_passwords_route))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
