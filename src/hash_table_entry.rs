pub struct Entry<Key, Value> {
    pub key: Key,
    pub value: Value,
    pub taken: bool,
}

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
