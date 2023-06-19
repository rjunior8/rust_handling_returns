use std::fmt::Error;
use std::sync::Arc;

struct Invalidation;

trait Raise {
    fn raise_invalid_user(&self) -> String;
}

impl Raise for Invalidation {
    fn raise_invalid_user(&self) -> String{
        return String::from("Invalid user!");
    }
}

pub fn get_status(username: &str) -> Option<&str> {
    // let users: Vec<&str> = vec!["Joao", "Pedro", "Maria"];
    // let users: [&str; 3] = ["Joao", "Pedro", "Maria"];
    let users = Arc::new(vec!["Joao", "Pedro", "Maria"]);

    if !users.iter().any(|&user| user == username) {
        return None;
    }

    Some(&username)
}

pub fn find_user(username: &str) -> Option<&str> {
    let found_user = get_status(username);
    match found_user {
        Some(user) => Some(user),
        None => None
    }
}

pub fn get_status_result(username: &str) -> Result<&str, String> {
    let users: Vec<&str> = vec!["Joao", "Pedro", "Maria"];

    if !users.iter().any(|&user| user == username) {
        return Err("Couldn't find user!".to_string());
    }

    Ok(&username)
}

pub fn find_user_result(username: &str) -> Result<&str, String>{
    let found_user = get_status_result(username);
    match found_user {
        Ok(user) => Ok(user),
        Err(e) => Err(e)
    }
}

pub fn validate_user(username: &str) -> &str {
    let _user = match get_status_result(username) {
        Ok(u) => {
            return u;
        },
        Err(_e) => panic!()
    };
}

pub fn if_user_exists(username: &str) -> &str {
    let user = get_status_result(username).unwrap();
    user
}

pub fn expect_user(username: &str) -> &str {
    let user = get_status_result(username).expect("Please provide a valid user.");
    user
}

pub fn get_user(username: &str) -> Result<&str, String> {
    let found_user = get_status_result(username);
    match found_user {
        Ok(user) => Ok(user),
        Err(_e) => Err(Invalidation::raise_invalid_user(&Invalidation))
    }
}

pub fn get_user_2(username: &str) -> Result<&str, String> {
    let found_user = get_status_result(username);
    match found_user {
        Ok(user) => Ok(user),
        Err(e) => Err(e)
    }
}

pub fn get_user_using_question_mark(username: &str) -> Result<&str, String> {
    let user = get_user(username)?;
    Ok(user)
}

pub fn to_borrow() -> String {
    let my_string: String = String::from("my_string");
    println!("Before to borrow: {}", my_string);
    to_take_borrowed(&my_string);
    println!("After to borrow: {}", my_string);
    println!("Borrowed successfully!");
    my_string
}

fn to_take_borrowed(value: &String) {
    println!("{}", value);
}
