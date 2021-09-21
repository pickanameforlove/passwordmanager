use rand::Rng;
use std::char;

use mysql::*;
use mysql::prelude::*;
use crossclip::{Clipboard, SystemClipboard};
use super::ui::list::password;

pub fn gen_password(length : u32) -> String{
    let mut password = String::new();
    let mut index = 0;

    while index < length {
        let letter : char = char::from_u32(rand::thread_rng().gen_range(33..127)).expect("enter the number of range 33 to 127");
        password.push(letter);
        index += 1;
    }
    password
}

pub fn gen_traditional_password(length : u32) -> String{
    let mut password = String::new();
    let mut index = 0;

    let mut chars = vec![];
    let up_char_len = 26;
    let up_char_start = 65;
    let char_len = 26;
    let char_start = 97;
    let digit_len = 10;
    let digit_start = 48;

    while index < up_char_len {
        let letter  = char::from_u32(up_char_start + index).expect("upchar error!");
        chars.push(letter);
        index += 1;
    }
    index = 0;
    while index < char_len {
        let letter  = char::from_u32(char_start + index).expect("char error!");
        chars.push(letter);
        index += 1;
    }
    index = 0;
    while index < digit_len {
        let letter  = char::from_u32(digit_start + index).expect("digit error!");
        chars.push(letter);
        index += 1;
    }
    index = 0;

    let total_len = up_char_len + char_len + digit_len;
    

    while index < length {
        let letter : char = chars[rand::thread_rng().gen_range(0..total_len) as usize];
        password.push(letter);
        index += 1;
    }
    password

}

pub fn insert_password(p:& mut password){
    let url = "mysql://root:123456@localhost:3306/passworddb";
    let opts = Opts::from_url(url).expect("mysql initialize Error!");

    let pool = match  Pool::new(opts){
        Ok(p) => p,
        Err(error) => panic!("Problem settting the pool: {:?}", error),
    };

    let mut conn = match pool.get_conn(){
        Ok(c) => c,
        Err(error) => panic!("Problem getting the connection: {:?}", error),
    };

    match conn.exec_drop(
        "INSERT INTO passwords (description, user, password)
        VALUES (:des, :u, :pass)", params! {
            "des" => &p.description,
            "u" => &p.user,
            "pass" => &p.password,
        }
    ) {
        Ok(_) => (),
        Err(error) => panic!("Problem execucing the statement: {:?}", error),
    };

    let pass_sec = p.password.clone();
    let clipboard =  SystemClipboard::new().unwrap();
    clipboard.set_string_contents(pass_sec).unwrap();
}