use hash_table::HashTable;

pub mod hash_trait;
pub mod hash_table;
pub mod hash_table_entry;

fn main() {
    println!("Hello, world!");

    // Demo of usage 
    let _table:HashTable<String, String> = HashTable::with_capacity(20);
}
