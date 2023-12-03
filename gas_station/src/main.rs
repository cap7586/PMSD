mod my_functions_gas_station_paez;

use my_functions_gas_station_paez::*;

use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    println!("This is the THI Student Paez Gas Station System");
    let mut inventory: Vec<Item> = Vec::new(); //vector of Items instances
    let my_file = "/Users/caropaez/Desktop/Universidad/PMSD/projects/my_gas_station/inventory_caro.csv";

    loop {
        print_menu();
        let in_operation = menu_get_user_input("Enter your choice:");
        match_operation(in_operation, &my_file, &mut inventory);
    }
}