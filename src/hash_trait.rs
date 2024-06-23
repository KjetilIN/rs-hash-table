pub trait Hash {
    fn hash(&self) -> usize;
}

impl Hash for String {
    fn hash(&self) -> usize {
        let mut hash = 5381;

        for byte in self.as_bytes() {
            hash = ((hash << 5) + hash) + (*byte as usize);
        }

        hash
    }
}
