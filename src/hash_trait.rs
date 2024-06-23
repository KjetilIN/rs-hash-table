/// Trait for the being able to hash a generic datatype into a `usize`
/// 
/// Uses the djb2 hash method for hashing 
pub trait Hash {
    /// Method that hashes the datatype into a `usize` 
    /// 
    /// Should implement the same hash method - djb2
    fn hash(&self) -> usize;
}

// Implementing the hash trait for string
impl Hash for String {
    fn hash(&self) -> usize {
        let mut hash = 5381;

        for byte in self.as_bytes() {
            hash = ((hash << 5) + hash) + (*byte as usize);
        }

        hash
    }
}
