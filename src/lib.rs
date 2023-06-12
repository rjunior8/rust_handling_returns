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
    let users: Vec<&str> = vec!["Joao", "Pedro", "Maria"];

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

pub fn get_user(username: &str) -> Result<&str, String>{
    let found_user = get_status_result(username);
    match found_user {
        Ok(user) => Ok(user),
        Err(_e) => Err(Invalidation::raise_invalid_user(&Invalidation))
    }
}

pub fn to_borrow() -> String {
    let my_string: String = String::from("my_string");
    println!("{}", my_string);
    to_owner(&my_string);
    println!("{}", my_string);
    my_string
}

fn to_owner(value: &String) {
    println!("{}", value);
}
