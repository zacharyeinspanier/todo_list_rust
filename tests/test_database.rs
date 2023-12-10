use todo_list_rust::database::database::TodoDatabase;
use todo_list_rust::todo::todo::TodoList;

#[test]
fn test_all(){
    let test_db = TodoDatabase::new(String::from("database/test_data.db"));
    print!("Test: test_user_create_remove...");
    test_user_create_remove(&test_db);
    println!("Pass");
    print!("Test: test_list_items_insert...");
    test_list_items_insert(&test_db);
    println!("Pass");

    match test_db.close_connection() {
        Err(err) => {panic!("{:?}", err);}
        Ok(()) => {},
    }
}


fn test_user_create_remove(test_db: &TodoDatabase){
    
    // Add a user
    match test_db.create_user_account("user_one", "pass", 1){
        Err(err) => {panic!("{:?}", err);}
        Ok(()) => {},
    };

    // Username UNIQUE ConstraintViolation
    match test_db.create_user_account("user_one", "pass", 2){
        Err(err) => {
            assert_eq!(err.sqlite_error_code().unwrap(), rusqlite::ErrorCode::ConstraintViolation);
        }
        Ok(()) => {panic!("Added user with duplicate username")},
    };

    // User_Id UNIQUE ConstraintViolation
    match test_db.create_user_account("user_two", "pass", 1){
        Err(err) => {
            assert_eq!(err.sqlite_error_code().unwrap(), rusqlite::ErrorCode::ConstraintViolation);
        }
        Ok(()) => {panic!("Added user with duplicate user_id")},
    };

    // Get user in database
    match test_db.get_user_id("user_one", "pass"){
        Err(err) => {panic!("{:?}", err)},
        Ok(result)=>{
            assert_eq!(1, result.len());
        },
    };
    
    // Get non existant user in database
    match test_db.get_user_id("no_user", "user_not_in_db"){
        Err(err) => {panic!("{:?}", err)},
        Ok(result)=>{
            assert_eq!(0, result.len());
        },
    };

    //Remove user
    match test_db.remove_user(1){
        Err(err) => {panic!("{:?}", err)},
        Ok(())=>{},
    };

    match test_db.get_user_id("user_one", "pass"){
        Err(err) => {panic!("{:?}", err)},
        Ok(result)=>{
            assert_eq!(0, result.len());
        },
    };
}


fn test_list_items_insert(test_db: &TodoDatabase){

    let test_db = TodoDatabase::new(String::from("database/test_data.db"));

    // User_Id UNIQUE ConstraintViolation
    match test_db.create_user_account("list_items_user", "pass", 2){
        Err(err) => {
            assert_eq!(err.sqlite_error_code().unwrap(), rusqlite::ErrorCode::ConstraintViolation);
        }
        _ => {},
    };

     // create list and items
     for list_id in 0..10{
        match test_db.insert_into_list(String::from("list") + &list_id.to_string(), list_id, 2){
            Err(err) => {panic!("{:?}", err)},
            Ok(())=>{},
        };
        for item_id in 0..10{
            match test_db.insert_into_items(String::from("item") + &item_id.to_string(), item_id + (list_id*100), list_id, 0){
                Err(err) => {panic!("{:?}", err)},
                Ok(())=>{},
            };
        }
    }

    // Check all lists were inserted
    let all_todos: Vec<TodoList> = test_db.load_user_data(2);
    assert_eq!(10, all_todos.len());

    //check all items were inserted
    for todo_list in all_todos{
        assert_eq!(10, todo_list.list.len());
        // remove an item
        test_db.remove_item(todo_list.get_item_id(0), todo_list.get_list_id());
    }

    // remove a list
    match test_db.remove_list(0, 2){
        Err(err) => {panic!("{:?}", err)},
        Ok(())=>{},
    };

    let all_todos_after_remove: Vec<TodoList> = test_db.load_user_data(2);
    assert_eq!(9, all_todos_after_remove.len());

    for todo_list in &all_todos_after_remove{
        assert_eq!(9, todo_list.list.len());
        // complete off items
        for item_index in 0..todo_list.get_list_len(){
            test_db.update_item(todo_list.get_item_id(item_index), todo_list.get_list_id(), 1);
        }
    }
    // check items were complete
    let all_todos_after_check_off: Vec<TodoList> = test_db.load_user_data(2);
    for todo_list in &all_todos_after_check_off{
        for item_index in 0..todo_list.get_list_len(){
            assert_eq!(true, todo_list.get_item_complete_status(item_index));
        }
    }
    
    // remove list and items
    for i in 0..10{
        match test_db.remove_list(i, 2){
            Err(err) => {panic!("{:?}", err)},
            Ok(())=>{},
        };
    }

    //Remove user
    match test_db.remove_user(2){
        Err(err) => {panic!("{:?}", err)},
        Ok(())=>{},
    };
    let user_removed: Vec<TodoList> = test_db.load_user_data(2);
    assert_eq!(0, user_removed.len());
}