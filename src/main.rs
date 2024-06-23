use hash_table::HashTable;

pub mod hash_table;
pub mod hash_table_entry;
pub mod hash_trait;

fn main() {
    // Demo of usage
    let table: HashTable<String, String> = HashTable::with_capacity(20);

    // Print the content of the hash table
    table.print();
}
