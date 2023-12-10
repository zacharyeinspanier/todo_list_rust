use todo_list_rust::user::user::User;

#[test]
fn test_user(){

    let mut user_test: User = User::new(1, String::from("username"), String::from("password"));
    assert_eq!(1, user_test.get_user_id());
    assert_eq!(String::from("username"), user_test.get_username());
    assert_eq!(String::from("password"), user_test.get_password());
    user_test.change_username(String::from("new_username"));
    assert_eq!(String::from("new_username"), user_test.get_username());
    user_test.change_password(String::from("new_password"));
    assert_eq!(String::from("new_password"), user_test.get_password());

}