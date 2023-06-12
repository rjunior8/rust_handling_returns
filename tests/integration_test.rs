use my_project;



const NONEXISTENT: &str = "non_existent";
const EXISTING: &str = "Joao";

#[test]
fn test_get_status_when_pass() {
    assert_eq!(Some(EXISTING), my_project::get_status(EXISTING));
}

#[test]
fn test_get_status_when_fail() {
    assert_eq!(None, my_project::get_status(NONEXISTENT));
}

#[test]
fn test_find_user_when_pass() {
    assert_eq!(Some(EXISTING), my_project::find_user(EXISTING));
}

#[test]
fn test_find_user_when_fail() {
    assert_eq!(None, my_project::find_user(NONEXISTENT));
}

#[test]
fn test_get_status_result_when_pass() {
    assert_eq!(Ok(EXISTING), my_project::get_status_result(EXISTING));
}

#[test]
fn test_get_status_result_when_fail() {
    assert_eq!(Err("Couldn't find user!".to_string()), my_project::get_status_result(NONEXISTENT));
}

#[test]
fn test_find_user_result_when_pass() {
    assert_eq!(Ok(EXISTING), my_project::find_user_result(EXISTING));
}

#[test]
fn test_find_user_result_when_fail() {
    assert_eq!(Err("Couldn't find user!".to_string()), my_project::find_user_result(NONEXISTENT));
}

#[test]
fn test_validate_user_when_pass() {
    let user = my_project::validate_user(EXISTING);
    assert_eq!(EXISTING, user);
}

#[test]
#[ignore]
fn test_validate_user_when_fail() {
    let user = std::panic::catch_unwind(|| {
        my_project::validate_user(NONEXISTENT)
    });
    assert!(user.is_err());
}

#[test]
#[should_panic]
fn test_validate_user_when_it_doesnt_exists() {
    let user = my_project::validate_user(NONEXISTENT);
}

#[test]
fn test_if_user_exists_when_pass() {
    let user = my_project::if_user_exists(EXISTING);
    assert_eq!(EXISTING, user);
}

#[test]
#[should_panic]
fn test_if_user_exists_when_it_doesnt_exists() {
    let user = my_project::if_user_exists(NONEXISTENT);
}

#[test]
fn test_expect_user_when_pass() {
    let user = my_project::expect_user(EXISTING);
    assert_eq!(EXISTING, user);
}

#[test]
#[should_panic]
fn test_expect_user_when_it_doesnt_exists() {
    let user = my_project::expect_user(NONEXISTENT);
}

#[test]
fn test_get_user_when_pass() {
    assert_eq!(Ok(EXISTING), my_project::get_user(EXISTING));
}

#[test]
fn test_get_user_when_fail() {
    assert_eq!(Err(String::from("Invalid user!")), my_project::get_user(NONEXISTENT));
}

#[test]
fn test_to_borrow_when_pass() {
    let my_string = my_project::to_borrow();
    assert_eq!(String::from("my_string"), my_string);
}
