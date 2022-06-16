pub mod transaction;

pub trait Hash {
    fn create_random_hash() -> String;
}