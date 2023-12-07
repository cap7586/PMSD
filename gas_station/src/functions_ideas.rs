//functions_ideas.rs

use std::io; //io library comes from the standard library std

////These lines import necessary modules from the standard library
use std::error::Error; //Used for error handling
use std::fs::File; //Used to work with files
use std::fs::{OpenOptions, rename};
use csv::{ReaderBuilder, WriterBuilder};//use csv::{ReaderBuilder, WriterBuilder};

/*impl Item {
    // Constructor method to create an Item instance from a StringRecord
    fn from_record(record: &StringRecord) -> Result<Item, Box<dyn Error>> {
        Ok(Item {
            id: record[0].parse()?,
            name: record[1].to_string(),
            price: record[2].parse()?,
            quantity: record[3].parse()?,
        })
    }

    // Method to check if the item has a specific ID
    /*fn has_id(&self, target_id: u32) -> bool {
        self.id == target_id
    }*/
}

#[derive(Clone)] 
pub struct Item {
    id: u32,
    name: String,
    price: f64,
    quantity: u32,
}
*/

/*
pub fn perform_sell(inventory: &Vec<Item>) {
    println!("Performing Sell");
    let item_id: u32 = get_user_input("Enter item ID:") as u32;
    let mut total_for_item = 0.0;
    if let Some(item) = inventory.iter().find(|&i| i.id == item_id) {
        let item_number: f64 = get_user_input("Enter quantity:");
        total_for_item = item.price * item_number;
        println!("Total for {}: {}", item.name, total_for_item);
    } else {
        println!("Item with ID {} not found in the inventory.", item_id);
    }
}
 */

/*pub fn display_inventory(inventory: &Vec<Item>) {
	//this function is printing the inventory for the user into the command line
    println!("Inventory:");
    for item in inventory {
        println!(
            "ID: {}, Name: {}, Price: {}, Quantity: {}",
            item.id, item.name, item.price, item.quantity
        );
    }
}*/

pub fn store_user_logs(operation_selected: usize, file_path_logs: &str, file_path_users: &str, user_account_name: &str, user_password: &str)-> Result<(), Box<dyn Error>> {
    let file = File::open(file_path_users)?;
    let mut rdr = csv::Reader::from_reader(file);

    let file_logs  = File::open(file_path_logs)?;
    let mut rdr_logs = csv::Reader::from_reader(file_logs);
    let mut row_count_logs = 0;

    for _ in rdr_logs.records(){
        row_count_logs += 1;
    }

    for result in rdr.records(){
        let record = result.unwrap();
        if user_account_name == &record[3] && user_password == &record[4] {
            let id_log = row_count_logs + 1;
            // Create a CSV writer in append mode
            let file_logs_to_write = OpenOptions::new().write(true).append(true).open(file_path_logs)?;
            let mut wtr = WriterBuilder::new().from_writer(file_logs_to_write);
            // Write a new record to the CSV file
            wtr.write_record(&[id_log,user_account_name,operation_selected.try_into().unwrap()])?;
            wtr.flush()?;
            return Ok(());
        }
    }
    Err("Error!".into())
}