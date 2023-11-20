pub mod todo{

    use crate::todo_item::todo_item::TodoItem;

    pub struct TodoList{
        name: String,
        list_id: u32,
        pub list: Vec<TodoItem>,
    }

    impl TodoList{

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
        pub fn item_complete_index(&mut self, index: usize) ->bool{
            // Check valid index
            if index >= self.list.len(){
                return false;
            }
            self.list[index].toggle_complete();
            // Set complete
            return self.list[index].get_complete();
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

        pub fn get_list_len(&self) -> usize{
            return self.list.len();

        }

        pub fn get_name(&self)->String{
            return self.name.clone();
        }
        pub fn get_list_id(&self) ->u32{
            return self.list_id;
        }
    }
}

