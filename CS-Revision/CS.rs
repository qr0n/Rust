use std::fs::File;
use std::io::prelude::*;

pub const MAX_ITEM_NAME_LENGTH: u32 = 20;
pub const MAX_CREDENTIAL_LENGTH: u32 = 20;

fn make_user(username : &str, password : &str) -> std::io::Result<()>
{
    let max_cred_length : usize = MAX_CREDENTIAL_LENGTH as usize;
    
    if username.chars().count() > max_cred_length || password.chars().count() > max_cred_length {
        println!("Hey! Your credentials are {} characters  bigger than the limit of {}!", username.chars().count() - max_cred_length, max_cred_length);
    }
    else {
        let file_name: String = format!("{}.txt", username);
        let mut file: File = File::create(file_name)?;
        
        let user_credentials: String  = format!("{}\n{}", username, password);
        
        let byte_user_credentials : &[u8] = user_credentials.as_bytes();
        
        file.write_all(byte_user_credentials)?;
    }
    Ok(())
}

fn credentials_exist(username : &str, password : &str) -> std::io::Result<()>
{
    let file_name: String = format!("{}.txt", username);
    let mut file: File = File::open(file_name)?;
    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content)?;

    let credential_content: String = format!("{}\n{}", username, password); 

    if credential_content == file_content {
        println!("Logged in");
    }
    Ok(())
}

fn get_menu() -> std::io::Result<()>
{
    let file_name: String = String::from("menu.txt");
    let mut file: File = File::open(file_name)?;
    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content)?;

    println!("Item\tPrice\tQuantity");
    println!("{}", file_content);

    Ok(())
}

fn add_to_menu(item_name: &str, item_price: i32, item_quantity: i32) -> std::io::Result<()>
{
    let file_name: String = String::from("menu.txt");
    let mut file: File = File::options().read(true).write(true).open(file_name)?;
    

    let mut previous_menu: String = String::new();
    file.read_to_string(&mut previous_menu)?;

    let menu_content: String = format!("{}\n{}\t{}\t{}", previous_menu, item_name, item_price, item_quantity);
    file.write_all(menu_content.as_bytes())?;
    
    println!("Current menu:");
    println!("Item\tPrice\tQuantity");
    println!("{}", menu_content);

    Ok(())
}

fn sell_item(item_name: &str, item_quantity: i32)
{
    
}

fn main() {
    // make_user("Hello", "World");             PASSED | TODO : Add match case for 'error handling' 
    // credentials_exist("foo", "1");           PASSED | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // get_menu();                              PASSED | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //add_to_menu("Test", 10, 10).expect("Ok"); PASSED | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

}