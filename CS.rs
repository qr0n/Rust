use std::fs::File;
use std::io::{prelude::*, Split};

static MAX_ITEM_NAME_LENGTH: u32 = 20;
static MAX_CREDENTIAL_LENGTH: u32 = 20;

struct MenuItem {
    name: String,
    price: i32,
    quantity: i32,
}

fn make_user(username : String, password : String) -> std::io::Result<()>
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

fn credentials_exist(username : String, password : String) -> std::io::Result<()>
{
    let file_name: String = format!("{}.txt", username);
    let mut file: File = File::open(file_name)?;
    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content);

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

    file.read_to_string(&mut file_content);

    println!("Item\tPrice\tQuantity");
    println!("{}", file_content);

    Ok(())
}

fn main() {
    // make_user(String::from(""), String::from(""));
    // credentials_exist(String::from("foo"), String::from("1"));
    get_menu();
}