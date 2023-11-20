pub mod todo_item{
    
    pub struct TodoItem{
        item_name: String,
        item_id: u32,
        date_created: String,
        date_complete: String,
        complete: bool,
    }
    impl TodoItem{


        pub fn new(item_name: String, item_id: u32, date_created: String) -> TodoItem{
            //let date_created = chrono::offset::Local::now().to_string(); 
            TodoItem{
                item_name, 
                item_id,
                date_created, 
                date_complete: String::from(""), 
                complete: false,
            }
        }

        pub fn new_from_load(item_name: String, item_id: u32, date_created: String, date_complete: String, complete: bool) -> TodoItem{ 
            TodoItem{
                item_name, 
                item_id,
                date_created, 
                date_complete, 
                complete,
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
        pub fn get_item_id(&self)->u32{
            return self.item_id;
        }

    }
}