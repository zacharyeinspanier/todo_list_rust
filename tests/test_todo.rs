use todo_list_rust::todo::todo::TodoList;

#[test]
fn test_todo(){
    //new
    let mut test_todo: TodoList = TodoList::new("test".to_string(), 100);

    // gets name and Id
    assert_eq!("test".to_string(), test_todo.get_name());
    assert_eq!(100, test_todo.get_list_id());


    // add items
    for i in 0..10{
        test_todo.add(String::from("item") + &i.to_string(), i);
    }
    assert_eq!(10, test_todo.get_list_len());

    // check item_id
    for i in 0..10{
        assert_eq!(i, test_todo.get_item_id(i as usize));
    }

    // check complete
    for i in 0..10{
        if i %2 == 0{
            test_todo.set_item_complete(i as usize);
            assert_eq!(true, test_todo.get_item_complete_status(i as usize));
        }
        
    }

     // remove all items
     for i in 0..10{
        test_todo.remove_index(0);
    }
    assert_eq!(0, test_todo.get_list_len());

    // check delete_list_items
    for i in 0..10{
        test_todo.add(String::from("item") + &i.to_string(), i);
    }
    assert_eq!(true, test_todo.delete_list_items());

    
    
    assert_eq!(0, test_todo.get_list_len());
    assert_eq!(false, test_todo.remove_index(0));
    assert_eq!(false, test_todo.set_item_complete(0));
    
    //set_item_complete
    //gets
}