use hash_table::HashTable;

pub mod hash_table;
pub mod hash_table_entry;
pub mod hash_trait;
pub mod unit_tests;

fn print_divider(){
    println!("\n ############################ \n"); 
}

fn print_get_result(key: String, op: Option<&String>){
    if let Some(item) = op{
        println!("GET:{:?} => {:?}", key, item);
    }else{
        println!("GET:{:?} => NONE", key);
    }
}

fn main() {
    // Demo of usage
    let mut table: HashTable<String, String> = HashTable::with_capacity(2);

    // Print the content of the hash table
    table.print();

    // Print divider
    print_divider();

    // Inserting a value => this insert should also extend the capacity by double 
    table.insert("12".to_string(), "AWS".to_string());
    table.insert("11".to_string(), "Microsoft".to_string());
    // Printing the new table content
    table.print();

    // Print divider
    print_divider();

    // Adding another value => Should not extend 
    table.insert("10".to_string(), "Apple".to_string());
    // Printing the new table content
    table.print();

    // Print divider
    print_divider();

    // Adding another value and this time it should extend the
    table.insert("9".to_string(), "NVIDIA".to_string());
    // Printing the new table content
    table.print();

    // Print divider
    print_divider();

    // Retrieve an element that does not exist 
    let key_google = "10".to_string();
    let google_item: Option<&String> = table.get(&key_google);
    print_get_result(key_google, google_item);

    // Retrieving an item that does exist 
    let key_aws = "12".to_string();
    let aws_item: Option<&String> = table.get(&key_aws);
    print_get_result(key_aws, aws_item);

    // Print divider
    print_divider();

    // Changing a value 



}
