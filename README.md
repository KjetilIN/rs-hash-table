# rs-hash-table
Implementation of hash table with Rust. Using open addressing for handling collision and djb2 as the hash function.


> ⚠️ Only for educational purposes. Use use `std::collections::HashMap`

# Usage 

Create a new `HashTable`:
```rust
let mut table: HashTable<String, String> = HashTable::with_capacity(2);
```

Insert a key value pair: 
```rust
table.insert("12".to_string(), "AWS".to_string());
```

Get the value by mutable reference or reference:
```rust
let aws_item: Option<&String> = table.get(&"10".to_string());
```

Delete a key value pair: 
```rust 
table.delete("10".to_string())
```

# Theory

## HashTable 
A hash table is a data structure that maps keys to values. It uses a hash function to compute an index of the given key within the array of values. The main reason why it is used, it because of the fast lookup time. You can find any element within the array with a big O notation `O(1)` (and worst is still `O(1)`). While a normal array is in worst case `O(n)`. 

## Open Addressing 
Open addressing is a method of collision resolution in hash tables. When a collision occurs (i.e., two keys hash to the same index), open addressing goes to the next empty slot in the array. In this code, we do linear probing to find the next empty slot. 

The code also extends the `HashTable` when the hashmap is 75% full. This is done when you insert a new key. For each insert, we check if we need to extend the hash table by one or not.  

## djb2 hashing 

djb2 is a simple hash function created by Daniel J. Bernstein. It's known for its good distribution properties and simplicity.

In the code each `Key` type must implement the `Hash` trait. The hash function is responsible for returning the djb2 hashed value of the `Key`

For example, here is the implementation of the trait for the `String` type (using djb2 hashing algorithm): 

```rust
impl Hash for String {
    fn hash(&self) -> usize {
        let mut hash = 5381;

        for byte in self.as_bytes() {
            hash = ((hash << 5) + hash) + (*byte as usize);
        }

        hash
    }
}
```



# Resources

Hash Table: <br>
https://en.wikipedia.org/wiki/Hash_table

Open Addressing: <br>
https://en.wikipedia.org/wiki/Open_addressing

djb2 hash function: <br>
http://www.cse.yorku.ca/~oz/hash.html 