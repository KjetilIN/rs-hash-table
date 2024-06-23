#[cfg(test)]
mod tests {
    use crate::hash_table::HashTable;

    #[test]
    fn test_with_capacity(){
        let _: HashTable<String, String> = HashTable::with_capacity(10);
    }

    #[test]
    fn test_insert_and_get(){
        let mut table: HashTable<String, String> = HashTable::with_capacity(10);
        let key = "0".to_string();
        let value = "AWS".to_string();

        // Insert 
        table.insert(key.clone(),value.clone());

        // Get value
        if let Some(val) = table.get(&key){
            assert_eq!(&value, val);
        }else{
            panic!("Was not able to find value")
        }
    }

    #[test]
    fn test_insert_and_mutate(){
        let mut table: HashTable<String, String> = HashTable::with_capacity(10);
        let key = "0".to_string();
        let value = "AWS".to_string();

        // Insert 
        table.insert(key.clone(),value.clone());

        // Get value
        if let Some(val) = table.get_mut(&key){
            assert_eq!(&value, val);
            // Reassign value
            *val = "MSC".to_string();

        }else{
            panic!("Was not able to find value")
        }

        // Try to get the new value
        if let Some(val) = table.get(&key){
            assert_eq!(&"MSC".to_string(), val);
        }else{
            panic!("Was not able to find value")
        }
    }


    #[test]
    fn test_delete(){
        let mut table: HashTable<String, String> = HashTable::with_capacity(10);
        let key = "0".to_string();
        let value = "AWS".to_string();

        // Insert 
        table.insert(key.clone(),value.clone());

        // Get value
        if let Some(val) = table.get_mut(&key){
            assert_eq!(&value, val);
        }

        // Delete value
        table.delete(key.clone());

        // Check if deleted correctly 
        if let Some(_) = table.get_mut(&key){
            panic!("Value was deleted, should be None")
        }
    }

}