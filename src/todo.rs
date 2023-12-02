pub mod todo{

    use crate::todo_item::todo_item::TodoItem;

    /*
        The structure for TodoList

        Members:
            name: String the name of the list
            list_id: u32 uniquly identify the todolist
            List: Vec of TodoItems, all items in the todo list
    */
    pub struct TodoList{
        name: String,
        list_id: u32,
        pub list: Vec<TodoItem>,
    }

    impl TodoList{

        /*
            Pram:
                list_name: String name of the todo list
                list_id: u32 used to uniquly identify the todolist
                
            Return: a new TodoList Struct
        */
        pub fn new(list_name: String, list_id: u32) ->TodoList{
            TodoList{
                name: list_name,
                list_id,
                list: Vec::new(),
            }
        }
        
        /*
            Add to the todo list
            Pram: string, name of TodoItem to create
            Return: bool, true if itme is added to list; flase otherwise
        */
        pub fn add(&mut self, item_name: String, item_id: u32, date_created: String) -> bool{        
            self.list.push(TodoItem::new(item_name, item_id, date_created));
            return true;
        }

        /*
            Remove from the todo list using index
            Pram: i32, index of TodoItem to remove
            Return: bool, true if item is removed; flase otherwise
        */
        pub fn remove_index(&mut self, index: usize)->bool{
            // Check valid index
            if index >= self.list.len(){
                return false;
            }
            // Remove
            self.list.remove(index.try_into().unwrap()); 
            return true;
        }

        /*
            Mark complete in the todo list using index
            Pram: i32, index of TodoItem to mark complete
            Return: bool, true if item is marked complete; false otherwise
        */
        pub fn set_item_complete(&mut self, index: usize, date_complete: String) ->bool{
            // Check valid index
            if index >= self.list.len(){
                return false;
            }

            self.list[index].toggle_complete();
            
            if !self.list[index].get_complete(){
                self.list[index].set_date_complete(String::from(""));
            }
            else{
                self.list[index].set_date_complete(date_complete.clone());
            }
            
            return true
        }

        /*
            Delete all TodoItems from a list
            Return: True if all times are deleted; flase otherwise
        */
        pub fn delete_list_items(&mut self)->bool{
            
            if self.list.len() == 0{
                return true
            }
            // Remove while not empty
            while !self.list.is_empty(){
                self.list.pop();
            }
            // List not emtpy
            if !self.list.is_empty(){
                return false;
            }
            return true;
        }
        /*
            Get item from todo list using string
            Pram: String, name of TodoItem
            Return: i32, index of the item, -1 if not found
        */
        fn get_item_by_name(&self, name: &String) -> i32{
            for (i, item) in self.list.iter().enumerate(){
                if &item.get_item_name() == name{
                    return i as i32;
                }
            }
            return -1;
        }

        /*
            Returns: usize the length of the todo list
        */
        pub fn get_list_len(&self) -> usize{
            return self.list.len();
        }
        /*
            Returns: String clone of the todo list name
        */
        pub fn get_name(&self)->String{
            return self.name.clone();
        }
        /*
            Returns: the todo list id
        */
        pub fn get_list_id(&self) ->u32{
            return self.list_id;
        }
        /*
            Pram: usize the item index
            Returns: the complete status of a specific todo item in the list
        */
        pub fn get_item_complete_status(&self, index: usize) -> bool{
            return self.list[index].get_complete();
        }
        /*
            Pram: usize the item index
            Returns: the item id for a specific todo item in the list
        */
        pub fn get_item_id(&self, index: usize) ->u32{
            return self.list[index].get_item_id();
        }
    }
}

