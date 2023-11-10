pub mod todotests{

    use crate::todo::todo::TodoList;

    pub fn test_add(n:i32, test_list:  &mut TodoList) ->bool{
    
        for i in (0..n).into_iter().map(|x| x.to_string()){
            if !test_list.add(String::from("item") + &i){
                return false;
            }
        }
        // dup check
        if test_list.add(String::from("item0")){
            return false;
        }
    
        if test_list.list.len() as i32 != n{
            return false
        }
        return true;   
    }
    
    pub fn test_remove(n:i32, test_list: &mut TodoList) ->bool {
        // remove by index
        if !test_list.remove_index(0){
            return false;
        }
        // remove by string
        if !test_list.remove_name(String::from("item1")){
            return false;
        }
        //check if gone
        if test_list.list.len() as i32 != n-2{
            return false;
        }
        // check item 1 was removed
        if test_list.remove_name(String::from("item1")){
            return false;
        }
        // remove invalid index
        if test_list.remove_index(100){
            return false;
        }
        for i in (2..n-1).into_iter().map(|x| x.to_string()){
            if !test_list.remove_name(String::from("item") + &i){
                return false;
            }
        }
    
        //check if gone
        if test_list.list.len() as i32 != 1{
            return false;
        }
    
        return true;
    
    }
    pub fn test_delete(test_list: &mut TodoList) -> bool{
        if test_list.delete_list_items(){
            return test_list.list.is_empty();
        }
        return false
    }
    
    pub fn test_complete(test_list: &mut TodoList)->bool{
        let list_size = test_list.list.len();
    
        for i in (0..list_size).into_iter().filter(|x| x %2 == 0){
            if !test_list.item_complete_index(i as i32){
                return false;
            }
        }
    
        test_list.add(String::from("string 1"));
        test_list.add(String::from("string 2"));
        test_list.add(String::from("string 3"));
        test_list.add(String::from("string 4"));
    
        if !test_list.item_complete_name(String::from("string 2")){
            return false;
        }
        if !test_list.item_complete_name(String::from("string 4")){
            return false;
        }
    
        for i in (0..list_size).into_iter().filter(|x| x %2 == 0){
            if !test_list.get_complete_status(i as i32){
                return false;
            }
        }
        return true;
    }
}