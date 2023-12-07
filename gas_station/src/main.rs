mod my_functions_gas_station;

use my_functions_gas_station::*;

use std::env;
use std::io;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    println!("This is the THI Student Paez Gas Station System");
    let inventory_file = "/Users/caropaez/Desktop/Universidad/PMSD/GitHub/PMSD/gas_station/Data/inventory_caro.csv";
    let user_data_file = "/Users/caropaez/Desktop/Universidad/PMSD/GitHub/PMSD/gas_station/Data/users.csv";

    let mut user_account_name_input = String::new();
    println!("Enter your User Name: ");
    io::stdin().read_line(&mut user_account_name_input).expect("Failed to read line");
    let user_account_name_input = user_account_name_input.trim();

    let mut user_account_password_input = String::new();
    println!("Enter your Password: ");
    io::stdin().read_line(&mut user_account_password_input).expect("Failed to read line");
    let user_account_password_input = user_account_password_input.trim();

    println!("{}", user_account_password_input);

    match check_user_password(&user_data_file, &user_account_name_input, &user_account_password_input) {
        Ok(()) => my_program_loop(&inventory_file),
        Err(err) => eprintln!("Error: {}", err),
    }
}