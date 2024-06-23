use std::fmt::Debug;

use crate::{hash_table_entry::Entry, hash_trait::Hash};

pub struct HashTable<Key, Value> {
    buckets: Vec<Entry<Key, Value>>,
    taken_count: usize,
    table_length: usize,
}

impl<Key, Value> HashTable<Key, Value>
where
    Key: Default + Hash + PartialEq + Debug,
    Value: Default + Debug,
{
    pub fn new() -> Self {
        HashTable {
            buckets: Vec::new(),
            taken_count: 0,
            table_length: 0,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size{
            buckets.push(Entry::default())
        }

        HashTable {
            buckets,
            taken_count: 0,
            table_length: size,
        }
    }

    pub fn get(&self, key: Key) -> Option<&Value> {
        let mut index = key.hash() % &self.table_length;

        while !self.buckets[index].taken && self.buckets[index].key != key {
            index = (index + 1) % self.table_length;
        }

        if self.buckets[index].taken {
            return Some(&self.buckets[index].value);
        }

        return None;
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut Value> {
        let mut index = key.hash() % &self.table_length;

        while !self.buckets[index].taken && self.buckets[index].key != key {
            index = (index + 1) % self.table_length;
        }

        if self.buckets[index].taken {
            return Some(&mut self.buckets[index].value);
        }

        return None;
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        todo!()
    }

    pub fn delete(&mut self, key: Key) {
        todo!()
    }

    pub fn print(&self) {
        for index in 0..self.table_length {
            let entry = &self.buckets[index];
            if entry.taken {
                let output = format!("'{:?}' => '{:?}'", entry.key, entry.value);
                println!("{}", output);
            } else {
                println!("' ' => ' '")
            }
        }
    }
}
