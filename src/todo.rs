pub mod todo{

    pub struct TodoItem{
        pub item_name: String,
        pub date_created: String,
        pub date_complete: String,
        pub complete: bool,

    }
    pub struct TodoList{
        pub name: String,
        pub list: Vec<TodoItem>,
    }

    impl TodoList{

        pub fn new(list_name: String) ->TodoList{
            TodoList{
                name: list_name,
                list: Vec::new(),
            }
        }
        /*
            Add to the todo list
            Pram: string, name of TodoItem to create
            Return: bool, true if itme is added to list; flase otherwise
        */
        pub fn add(&mut self, item_name: String) -> bool{
            // Check for duplicates; Not allowed
            let res: i32 = self.get_item_by_name(&item_name);
            if res != -1 {
                return false;
            }
            // Create TodoItem
            let date_created: String = chrono::offset::Local::now().to_string(); 
            let new_item = TodoItem{
                item_name, 
                date_created, 
                date_complete: String::from("Not Complete"), 
                complete: false,
            };
            self.list.push(new_item);
            return true;
        }

        /*
            Remove from the todo list using index
            Pram: i32, index of TodoItem to remove
            Return: bool, true if item is removed; flase otherwise
        */
        pub fn remove_index(&mut self, index: i32)->bool{
            // Check valid index
            if index >= self.list.len() as i32 || index < 0{
                return false;
            }
            // Remove
            self.list.remove(index.try_into().unwrap()); 
            return true;
        }

        /*
            Remove from the todo list using string
            Pram: String name of TodoItme to remove
            Return: bool, true if item is removed; flase otherwise
        */
        pub fn remove_name(&mut self, item_name: String)->bool{
            // Get index
            let res: i32 = self.get_item_by_name(&item_name);
            // Check valid index
            if res < 0 || res >= self.list.len() as i32 {
                return false;
            }
            // Remove
            self.list.remove(res.try_into().unwrap()); 
            return true;
        }

        /*
            Mark complete in the todo list using string
            Pram: String, name of TodoItem to mark complete
            Return: bool, true if TodoItme is marked complete; false otherwise
        */
        pub fn item_complete_name(&mut self, item_name: String) ->bool{
            // Get index
            let res: i32 = self.get_item_by_name(&item_name);
            // Check valid index
            if res < 0 || res >= self.list.len() as i32 {
                return false;
            }
            // Set complete
            return self.list[res as usize].set_complete();
        }

        /*
            Mark complete in the todo list using index
            Pram: i32, index of TodoItem to mark complete
            Return: bool, true if item is marked complete; false otherwise
        */
        pub fn item_complete_index(&mut self, index: i32) ->bool{
            // Check valid index
            if index >= self.list.len() as i32 || index < 0{
                return false;
            }
            // Set complete
            return self.list[index as usize].set_complete();
        }

        /*
            Get complete status from todo list using index
            Pram: i32, index of the TodoItem in the list
            Return: bool, true if TodoItem is complete; flase otherwise
        */
        pub fn get_complete_status(&mut self, index: i32) -> bool{
            // Check valid index
            if index >= self.list.len() as i32 || index < 0{
                return false;
            }
            // Status
            return self.list[index as usize].complete;
        }
        /*
            Creates string to print all TodoItems in the list
            Return: String Formatting for all Todo Times
        */
        pub fn print_list(&self) -> String{
            // Put list name
            let mut all_list_times = format!("######{}######\n", self.name);

            if self.list.len() as i32 != 0{
                // Add TodoItem to string
                for item in &self.list{
                    all_list_times.push_str(&item.item_name);
                    all_list_times.push(' ');
                    all_list_times.push_str(&item.complete.to_string());
                    all_list_times.push_str("\n");
                }
            }

            return all_list_times;
        }
        /*
            Delete all TodoItems from a list
            Return: True if all times are deleted; flase otherwise
        */
        pub fn delete_list_items(&mut self)->bool{
            
            if self.list.len() as i32 == 0{
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
                if &item.item_name == name{
                    return i as i32;
                }
            }
            return -1;
        }
    }

    impl TodoItem{
        /*
            Set complete status of a TodoItem
            Return: bool, true if successful; flase otherwise
        */
        fn set_complete(&mut self) -> bool{
            if !self.complete{
                self.complete = true;
                return true
            }
            return false;
        }
    }
}

