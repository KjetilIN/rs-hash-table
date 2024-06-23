pub struct Entry<Key, Value>{
    pub key: Key,
    pub value: Value,
    pub taken: bool
}