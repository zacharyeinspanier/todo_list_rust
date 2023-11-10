
mod todo;
mod todotests;
use crate::todo::todo::TodoList;
use crate::todotests::todotests::*;

fn run_tests(){
    let mut test_list = TodoList{
        name: String::from("Test List"),
        list: Vec::new(),
    }; 
   
    if test_add(100, &mut test_list) {
        println!("Add to list Pass")
    }
    else{
        println!("Add to list Fail")
    }

    if test_remove(100, &mut test_list) {
        println!("Remove to list Pass")
    }
    else{
        println!("Remove to list Fail")
    }
    // Add itmes for delete
    test_add(10, &mut test_list);

    if test_complete(&mut test_list){
        println!("Complete to list Pass");
    }
    else{
        println!("Complete to list Fail");
    }

    if test_delete(&mut test_list){
        println!("Delete to list Pass");
    }
    else{
        println!("Delete to list Failed");
    }
    println!("{}", test_list.print_list());
    
}

fn main(){
    run_tests();
}
