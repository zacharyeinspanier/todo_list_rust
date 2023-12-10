use todo_list_rust::todo_item::todo_item::TodoItem;


#[test]
fn test_todo_item(){
    let mut item_one: TodoItem = TodoItem::new(String::from("item_one"), 1);
    assert_eq!(String::from("item_one"), item_one.get_item_name());
    assert_eq!(1, item_one.get_item_id());
    assert_eq!(false, item_one.get_complete());
    item_one.toggle_complete();
    assert_eq!(true, item_one.get_complete());

    let mut item_two: TodoItem = TodoItem::new_from_load(String::from("item_two"), 2, true);
    assert_eq!(String::from("item_two"), item_two.get_item_name());
    assert_eq!(2, item_two.get_item_id());
    assert_eq!(true, item_two.get_complete());
    item_two.toggle_complete();
    assert_eq!(false, item_two.get_complete());
}

