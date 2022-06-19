use rand::{thread_rng,Rng};
use rand::rngs::ThreadRng;
use rand::distributions::Alphanumeric;

pub mod transaction;
pub mod action;

pub trait Hash {
    fn create_random_hash(hash_length: usize) -> () {
        let random_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(hash_length)
            .map(char::from)
            .collect();
        println!("{:?}",random_string)
    }
}