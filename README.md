#Principles of Modern Software Development

The Gas Station Exercise by Caro Paez


Fhe Gas Station Program can perform the following operations:
Sell x quantity of a product to a customer,
Add a new Product to the System,
Add more quantity of a Product that is already existing in the Inventory,
Remove Product Stock from the Inventory,
Update Product Price,
Print the Inventory in the screan,
Remove completely a Product from the Inventory,
Exit the program


impl Item:
Is the Constructor method to create an Item instance from a StringRecord
    fn from_record(record: &StringRecord) -> Result<Item, Box<dyn Error>>

Item is a struct which contains the ID, name, price and quantity of our "product".

Function get_user_input:
is used to read the input from the keyboard of the user

Function read_csv:
gets a file path and opens it 
then prints 1 line which represents the header of our csv file
and then loops through all the records (rows) of our csv file and prints them

Function write_csv_from_user:
This function is used to actually write new instances (rows) into the csv file that is passed as parameter (file_path). It can be used to add complete new products to the inventory.
It is used by the function "add_product_to_inventory"


Function add_product_to_inventory:
This function allows the user to add a new product to the inventory
It creates an instance of the Item struct
Then it asks the user to add the product name, its id, price and quantity and adds it as the values of the variables of this struct.
Then it adds a new instance (row) of this item to the csv file that contains the inventory, this way we can have the inventory stored outside the program

Function print_menu:
This function is only printing the menu for the user in the command line

Function match_operation:
This is the "back end" to our menu displayed to the user.
It is going to take care of implementing the right method, depending on the selection made by the user

Function menu_get_user_input:
Similar to the get_user_input function, but it returns a different kind of value (usize)

Function get_id_and_then_delete:
Gets the id of a product that the user wants to delete and then executes the method delete_product_by_id that actually deletes this product. 
It is needed because the delete_product_by_id is returning something but we dont want this into our match_operation function

Function delete_product_by_id:
It opens a csv file, then it creates a new csv called "temp" because we are going to use this to write all the "rows" (products) but not the one we want to remove into this file, and then we can replace our old file which contains the element we want to delete with the new file by just renaming it

Function get_id_and_then_update:
Gets the id of a product that the user wants to update and then executes the method add_new_product_quantity that actually updates the quantity of this product. 
It is needed because the add_new_product_quantity is returning something but we dont want this into our match_operation function

Function add_new_product_quantity:
Gets the id of the product to update as a parameter. The user inputs the quantity. Uses get_id_and_then_update, delete_product_by_id and write_csv_from_user.

Function lets_sell_something:
Gets the id of the product we want to sell as a parameter. Uses get_user_input to get the quantity to be sold and then checks if we actually have this quantity in the inventory. 
//next feature -> has to update the inventory after performing the sell
Then it calculates the total for sellint that quantity of the product and prints that to the user, together with the unit price.
If there is not enough quantity of this product in the inventory, it shows a warning to the user and displays the actual amount of this product available in the inventory

Function get_id_and_then_sell:
Gets the id of a product that the user wants to sell and then executes the method lets_sell_something that actually calculates the total for this product. 
It is needed because the lets_sell_something is returning something but we dont want this into our match_operation function

Function get_id_and_then_update_price:
Gets the id of a product that the user wants to update and then executes the method change_product_price that actually updates the price of this product. 
It is needed because the change_product_price is returning something but we dont want this into our match_operation function

Function change_product_price:
Gets the id as a parameter. Reads the csv file. Gets the new price from the user. 
Deletes this instance (row) from the csv file and then writes it as a new item with the new price into our file.