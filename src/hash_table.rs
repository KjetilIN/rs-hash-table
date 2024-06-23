use std::fmt::Debug;

use crate::{hash_table_entry::Entry, hash_trait::Hash};


/// HashTable data structure. 
/// 
/// Read more about how this data structure is implemented in the [README](readme.md)
pub struct HashTable<Key, Value> {
    buckets: Vec<Entry<Key, Value>>,
    taken_count: usize,
    table_length: usize,
}

impl<Key, Value> HashTable<Key, Value>
where
    Key: Default + Hash + PartialEq + Debug + Clone,
    Value: Default + Debug + Clone,
{
    /// Creates a new `HashTable` with given capacity 
    /// 
    /// Fills the buckets with empty `Entry` values. 
    /// That is not taken.
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

    /// Private function that finds a slot for the given key
    /// 
    /// Uses the hashed value of the key.
    /// Uses open addressing to find the key slot that is not taken. 
    fn find_slot(&self, key: &Key) -> usize{
        let mut index = key.hash() % &self.table_length;

        while self.buckets[index].taken && self.buckets[index].key != *key {
            index = (index + 1) % self.table_length;
        }

        index
    }

    /// Get value (as reference) from the key
    /// 
    /// Returns an option based on if the key is present in the `HashTable`
    pub fn get(&self, key: &Key) -> Option<&Value> {
        let index = self.find_slot(key);

        if self.buckets[index].taken {
            return Some(&self.buckets[index].value);
        }

        return None;
    }

    /// Get a mutable reference to the value, given the key.
    /// 
    /// Returns an option based on if the key is present in the `HashTable`
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        let index = self.find_slot(key);

        if self.buckets[index].taken {
            return Some(&mut self.buckets[index].value);
        }

        return None;
    }

    /// Insert a new key value pair into the `HashTable`
    /// 
    /// Will extend the `HashTable` if there is not enough room for new elements.
    /// We extend the if there is the `HashTable` is filled more than 75% of its capacity
    pub fn insert(&mut self, key: Key, value: Value) {
        let index = self.find_slot(&key);

        // Check if we need to extend the hash table 
        if self.taken_count + 1 > self.table_length * 3/4{
            self.extend()
        }

        // Set value 
        self.buckets[index].taken = true;
        self.buckets[index].key = key;
        self.buckets[index].value = value;

        self.taken_count += 1
    }

    /// Function that extends the `HashTable` by creating a new one
    /// 
    /// The new `HashTable` is twice the size of the original one.
    fn extend(&mut self){
        let new_size = self.table_length * 2; 
        let mut new_table: HashTable<Key, Value> = HashTable::with_capacity(new_size);

        // Iterate over all keys in the table and insert one by one
        for entry in &self.buckets{
            if entry.taken{
                new_table.insert(entry.key.clone(), entry.value.clone());
            }
        }

        *self = new_table;
    }


    /// Delete a key value pair 
    /// 
    /// If the key is found, reset the values to defaults. 
    /// Compresses the `HashTable` if:
    /// - The total length is more than 50 and
    /// - Only 33% of the `HashTable` is occupied 
    pub fn delete(&mut self, key: Key) {
        let index = self.find_slot(&key);

        // Delete value if taken 
        if self.buckets[index].taken{
            self.buckets[index].taken = false;
            self.buckets[index].key = Default::default();
            self.buckets[index].value = Default::default();
            self.taken_count -= 1; 
        }

        // Check if we need to compress the table. 
        if self.table_length > 50 && self.taken_count * 3 < self.table_length {
            self.compress();
        }
    }

    /// Function for compressing the table into half its length 
    /// 
    /// Creates a new `HashTable` and populate it with all the key value pairs in the buckets. 
    fn compress(&mut self){
        let new_size = self.table_length / 2; 
        let mut new_table: HashTable<Key, Value> = HashTable::with_capacity(new_size);

        // Iterate over all keys in the table and insert one by one
        for entry in &self.buckets{
            if entry.taken{
                new_table.insert(entry.key.clone(), entry.value.clone());
            }
        }

        *self = new_table;

    }

    /// Print function that prints the content of the function 
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
