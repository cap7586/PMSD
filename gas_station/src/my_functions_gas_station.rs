//my_functions_gas_station_paez.rs

use std::io; //io library comes from the standard library std

//These lines import necessary modules from different libraries
use std::error::Error; //Used for error handling
use std::fs::File; //Used to work with files
use std::fs::{OpenOptions, rename};
use csv::{ReaderBuilder, WriterBuilder};

pub enum Operation {
    Sell,
    AddProductToGasStation,
    AddProductStockToInventory,
    RemoveProductStockFromInventory,
    UpdateProductPrice,
    SeeInventory,
    RemoveProductFromInventory,
    Exit,
}

/*
My first idea was to use a struct, but then I thought it would be better to store all the data in csv files in order to have them still available 
even after termination of the program execution

#[derive(Clone)] 
pub struct Item {
    id: u32,
    name: String,
    price: f64,
    quantity: u32,
}
*/

pub fn get_user_input(prompt: &str) -> f64 {
	//this function is used to read the input from the keyboard of the user

    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid number entered")
}

pub fn read_csv(file_path: &str) -> () {
    let file = File::open(file_path).unwrap();//This line attempts to open the file specified by the file_path. The ? operator is used to propagate any errors that may occur during the file opening process.

    let mut rdr = csv::Reader::from_reader(file);

    println!("ID | Name | Price | Quantity ");
    for result in rdr.records() {
        let record = result.unwrap();

       // println!("{:?}", record.iter().collect::<Vec<_>>());

	    let id = record[0].parse::<u32>().unwrap();
	    let name = &record[1].trim();
	    let price = record[2].parse::<f64>().unwrap();
	    let quantity = record[3].parse::<u32>().unwrap();

    	println!("{}, {}, {}, {}", id, name, price, quantity);
    }
}

pub fn write_csv_from_user(file_path: &str, id_input: &str, name_input:&str, price_input:&str, quantity_input:&str) -> Result<(), Box<dyn Error>>{
	//this function is used to actually write new instances (rows) into the csv file that is passed as parameter (file_path)
    
    // Create a CSV writer in append mode
    let file = OpenOptions::new().write(true).append(true).open(file_path)?;

    let mut wtr = WriterBuilder::new().from_writer(file);

    // Write a new record to the CSV file
    wtr.write_record(&[id_input,name_input,price_input,quantity_input])?;
    wtr.flush()?;

    Ok(())
}

pub fn add_product_to_inventory(file_path: &str){
	//this function allows the user to add a new product to the inventory
    let mut id: u32 = 0;
    let mut name: String = String::new();
    let mut price: f64 = 0.0;
    let mut quantity: u32 = 0;

    //then it asks the user to add the product name, its id, price and quantity
    println!("Product Name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    id = get_user_input("Enter product ID:") as u32;
    price = get_user_input("Enter product price:");
    quantity = get_user_input("Enter product quantity:") as u32;

    //and then it adds a new instance (row) of this item to the csv file that contains the inventory, this way we can have the inventory stored outside the program
    write_csv_from_user(file_path,&id.to_string(),&name.trim(),&price.to_string(),&quantity.to_string())
    .expect("Failed to write CSV");
}

pub fn print_menu(){
	//This function is only printing the menu for the user in the command line
        println!("\n\nSelect the operation you want to perform");
        println!("1. Sell Something");
        println!("2. Change Product Stock");
        println!("3. Add a new Product to the System");
        println!("4. Coming Soon Feature");//not there yet
        println!("5. Update a Product Price");
        println!("6. See Inventory");
        println!("6b. See One Product");//not implemented yet
        println!("7. Delete Product by ID");
        println!("99. Exit\n");
}

pub fn match_operation(choice: usize, file_path: &str) {
    let selected_operation: Operation = match choice {
        1 => Operation::Sell,
        2 => Operation::AddProductStockToInventory,
        3 => Operation::AddProductToGasStation,
        4 => Operation::RemoveProductStockFromInventory,
        5 => Operation::UpdateProductPrice,
        6 => Operation::SeeInventory,
        7 => Operation::RemoveProductFromInventory,
        99 => Operation::Exit,
        _ => {
            println!("Invalid input");
            return;
        }
    };

    match selected_operation {
        Operation::Sell => get_id_and_then_sell(file_path),
        Operation::AddProductStockToInventory => get_id_and_then_update(file_path),//add_new_product_quantity(file_path),
        Operation::AddProductToGasStation => add_product_to_inventory(file_path),
        Operation::RemoveProductStockFromInventory => println!("Performing RemoveProductStockFromInventory"),
        Operation::UpdateProductPrice => get_id_and_then_update_price(file_path),
        Operation::SeeInventory => read_csv(file_path),//display_inventory(inventory),
        Operation::RemoveProductFromInventory => get_id_and_then_delete(file_path),
        Operation::Exit => {
            println!("Exiting the THI Student Paez Gas Station System");
            std::process::exit(0);
        }
    }
}

pub fn menu_get_user_input(prompt: &str) -> usize {
	//get user input from the keyboard and 
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

pub fn get_id_and_then_delete(file_path: &str){
	let id_to_delete_from_usr = get_user_input("Enter Product ID to delete: ") as u32;
	let _ = delete_product_by_id(file_path,id_to_delete_from_usr);
}

pub fn delete_product_by_id(file_path: &str, id: u32) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let temp_file_path = "temp.csv";  // You can adjust the temporary file path as needed
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the header from the CSV file
    let _header = rdr.headers()?.clone();

    // Create a CSV writer in write mode to update the file
    let _file = OpenOptions::new().write(true).open(file_path)?;
    let temp_file = OpenOptions::new().write(true).create(true).truncate(true).open(&temp_file_path)?;
    let mut wtr = WriterBuilder::new().from_writer(temp_file);

    // Read the header from the CSV file
    let header = rdr.headers()?.clone();

    // Write back the header to the temporary CSV file
    wtr.write_record(header.iter())?;

    for result in rdr.records() {
        let record = result?;
       	if record.len() == 4 && record[0].parse::<u32>().unwrap() != id {
        // Write only if the length is 4 and the ID does not match the one to delete
        	wtr.write_record(&record)?;
    	}

    }

    wtr.flush()?;

    // Replace the old file with the new one
    rename(&temp_file_path, file_path)?;

    Ok(())
}

pub fn get_id_and_then_update(file_path: &str){
	let id_to_update_from_usr = get_user_input("Enter Product ID to UPDATE: ") as u32;
	let _ = add_new_product_quantity(file_path,id_to_update_from_usr);
}

pub fn add_new_product_quantity(file_path: &str, id: u32)-> Result<(), Box<dyn Error>> {
    // Create a CSV reader
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let new_quantity = get_user_input("Add new product quantity: ") as u32; //asks the user for the new quantity

    for result in rdr.records() {
        let record = result.unwrap();
        let id_checker = record[0].parse::<u32>().unwrap();
        let item_name = &record[1];
    	let item_id = id;
    	let item_price = &record[2];
    	let item_quantity = new_quantity;
    	let mut aux = 0;        
    	if id_checker == id{
    		let _ = delete_product_by_id(file_path,id);
    		println!("It was deleted");
    		aux = 99;
        }
        if aux == 99{
        	let _ = write_csv_from_user(file_path,&item_id.to_string(),&item_name,&item_price.to_string(),&item_quantity.to_string());
        	println!("It was written");

        }
    }
    Ok(())
}

pub fn lets_sell_something(file_path: &str, id: u32)-> Result<(), Box<dyn Error>> {
	let file = File::open(file_path)?;
	let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut total_for_product: u32;

	let quantity_input_user = get_user_input("Enter Quantity: ") as u32;

	for result in rdr.records() {
		let record = result?;
       	if record[0].parse::<u32>().unwrap() == id && record[3].parse::<u32>().unwrap() >= quantity_input_user{
        	total_for_product=record[2].parse::<u32>().unwrap() * quantity_input_user;
            println!("Unit Price: {:?}", record[2].parse::<u32>().unwrap());
            println!("Total for {:?}: {}", &record[1],total_for_product);
    	}
    	else if record[0].parse::<u32>().unwrap() == id && record[3].parse::<u32>().unwrap() < quantity_input_user {
    		println!("There is not enough quantity of this product in the inventory. Current amount of this product available is: {:?}", &record[3]);
    		break;
    	}
    }

	Ok(())
}

pub fn get_id_and_then_sell(file_path: &str){
	let id_to_sell_from_usr = get_user_input("Enter Product ID: ") as u32;
	let _ = lets_sell_something(file_path,id_to_sell_from_usr);
}

pub fn get_id_and_then_update_price(file_path: &str){
	let id_to_update_from_usr = get_user_input("Enter Product ID to UPDATE: ") as u32;
	let _ = change_product_price(file_path,id_to_update_from_usr);
}

pub fn change_product_price(file_path: &str, id: u32)-> Result<(), Box<dyn Error>> {
    // Create a CSV reader
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let new_price: f32 = get_user_input("Add new product price: ") as f32; //asks the user for the new quantity

    for result in rdr.records() {
        let record = result.unwrap();
        let id_checker = record[0].parse::<u32>().unwrap();
        let item_name = &record[1];
    	let item_id = id;
    	let item_price = new_price;
    	let item_quantity = &record[3];
    	let mut aux = 0;        
    	if id_checker == id{
    		let _ = delete_product_by_id(file_path,id);
    		aux = 99;
        }
        if aux == 99{
        	let _ = write_csv_from_user(file_path,&item_id.to_string(),&item_name,&item_price.to_string(),&item_quantity.to_string());
        }
    }
    Ok(())
}

pub fn check_user_password(file_path: &str, user_account_name: &str, user_password: &str)-> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records(){
        let record = result.unwrap();
        if user_account_name == &record[3] && user_password == &record[4] {
            return Ok(());
        }
    }
    Err("Incorrect combination of user name and password".into())
}

pub fn my_program_loop(inventory_csv: &str){
    loop {
        print_menu();
        let in_operation = menu_get_user_input("Enter your choice:");
        match_operation(in_operation, &inventory_csv);
    }
}
