use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;
use std::str::FromStr;
use std::fmt;

// HASH TYPE
#[derive(Clone, PartialEq, Eq)]
struct Hash32([u8; 32]);
  // FROM STR (Hex -> Bytes)
impl FromStr for Hash32 {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s.trim())?;

        let hash_array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| hex::FromHexError::InvalidStringLength)?;

        Ok(Hash32(hash_array))
    }
}

// Display (Bytes -> Hex String)
impl fmt::Display for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0)) 
    }
}
impl Hash32 {
    fn hash_password(password: &str) -> Self {
    let digest = Sha256::digest(password.as_bytes());
    Hash32(digest.into())
}
    
}
// HASH FUNCTION (passsword -> Hash32)
fn hash_password(password: &str) -> Hash32 {
    let digest = Sha256::digest(password.as_bytes());
    Hash32(digest.into())
}

// MAIN FUNCTION
fn main() {
   let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid amount of arguments!");
        println!("Example: cargo run <input>");
        exit(1);
    }
    // CONVERT INPUT STRING -> Hash32 using FromStr
    let inserted_hash: Hash32 = args[1].parse().unwrap();
    let password_file = "passwords.txt";

    // OPEN PASSWORD FILE
    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);
    // SEARCH LOOP
     let result = reader
        .lines()
        .filter_map(|pass| pass.ok())
        .find(|password| {
          Hash32::hash_password(password) == inserted_hash
        });
    // OUTPUT
    match result {
        Some(password ) => {
            println!("Match found!");
            println!("Password: {}", password);
            println!("Hash: {}", hash_password(&password));
        }
        None => {
            println!("No match found.");
        }
    }
}
