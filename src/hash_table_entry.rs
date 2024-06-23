
/// Data structure that represents a single entry in the `HashTable`
/// 
/// - key, value is the key-value pair for the entry
/// - taken is true when there is a value in the entry 
pub struct Entry<Key, Value> {
    pub key: Key,
    pub value: Value,
    pub taken: bool,
}


// Implementing `Default` trait for Entry
impl<Key, Value> Default for Entry<Key, Value>
where
    Key: Default,
    Value: Default,
{
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            taken: false,
        }
    }
}
