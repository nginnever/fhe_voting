mod util;

use std::{env, error::Error, process::exit, sync::Arc};
use console::style;
use fhe::{
    bfv::{self, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("casting encrypted vote");
    Ok(())
}
