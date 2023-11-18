pub mod todo_item{
    pub struct TodoItem{
        item_name: String,
        date_created: String,
        date_complete: String,
        complete: bool,
    }
    impl TodoItem{


        pub fn new(item_name: String) -> TodoItem{
            let date_created = chrono::offset::Local::now().to_string(); 
            TodoItem{
                item_name, 
                date_created, 
                date_complete: String::from(""), 
                complete: false,
            }
        }
        /*
            Set complete status of a TodoItem
            Return: bool, true if successful; flase otherwise
        */
        pub fn toggle_complete(&mut self){
            self.complete = !self.complete;
            if self.complete{
                self.set_date_complete();
            }
            else{
                self.date_complete = String::from("");
            }
        }

        pub fn get_item_name(&self) ->String{
            return self.item_name.clone();
        }
        pub fn get_date_created(&self) ->String{
            return self.date_created.clone();

        }
        pub fn get_date_complete(&self)-> String{
            return self.date_complete.clone();

        }
        pub fn set_date_complete(&mut self){
            self.date_complete = chrono::offset::Local::now().to_string(); 
        }
        pub fn get_complete(&self) -> bool{
            return self.complete;
        }

    }
}