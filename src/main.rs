use std::io;
use crossclip::{Clipboard, SystemClipboard};
use mysql::*;
use mysql::prelude::*;
use eframe::{egui, epi};

mod addPassword;
mod ui;

fn main(){
    println!("what do you want to do ? 0 for adding password,1 for showing passwords");
    let mut op_num = String::new();
    io::stdin().read_line(&mut op_num).expect("Failed to read!");
    let op_num : u32 = match op_num.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 1,
    };
    if op_num == 0 {
        add_password();
    }else if op_num == 1 {
        let app = ui::MyApp{data : vec![],pasd : String::new()};
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(Box::new(app), native_options);
    }else{
        println!("there is some aspects not thinking in this program!");
    }
    

}
fn add_password(){
    println!("please enter the website url or some description.");
    let mut website = String::new();
    io::stdin().read_line(&mut website).expect("Failed to read!");
    website = website.trim().to_string();
    

    println!("do you want produce a traditional password? 0 for No, 1 or no input for Yes");
    let mut if_num = String::new();
    io::stdin().read_line(&mut if_num).expect("Failed to read!");
    let if_num : u32 = match if_num.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 1,
    };

    println!("please enter the username.");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read!");
    username = username.trim().to_string();
    

    println!("please the length of password! the default length is 18");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Failed to read length!");

    let length:u32 = match length.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 18,
    };
    
    let pass = if if_num == 1 {
        addPassword::gen_traditional_password(length)
    }else{
        addPassword::gen_password(length)
    };

    println!("the password is {}", pass);

    let url = "mysql://root:123456@localhost:3306/passworddb";
    let opts = Opts::from_url(url).expect("mysql initialize Error!");

    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.exec_drop(
        "INSERT INTO passwords (description, user, password)
         VALUES (:des, :u, :pass)", params! {
             "des" => &website,
             "u" => &username,
             "pass" => &pass,
         }
    );

    let pass_sec = pass.clone();
    let clipboard =  SystemClipboard::new().unwrap();
    clipboard.set_string_contents(pass_sec).unwrap();
}




// fn get_time_humanReadable() -> String{
//     // Creates a new SystemTime from the specified number of whole seconds
//     let d = SystemTime::now();
//     // Create DateTime from SystemTime
//     let datetime = DateTime::<Local>::from(d);
//     // Formats the combined date and time with the specified format string.
//     datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
// }



