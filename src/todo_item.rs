pub mod todo_item{


    /*
        This structue hold data about TodoItem

        Members:
            item_name: String, item name
            item_id: u32, unique identifier for item
            date_created: String, item date created
            date_complete: String, item complete date
            complete: bool, complete status true = complete false = not complete


    */
    pub struct TodoItem{
        item_name: String,
        item_id: u32,
        date_created: String,
        date_complete: String,
        complete: bool,
    }
    impl TodoItem{

        /*
            Create new todo item item
            Prams:
                item_name: String name for the item
                item_id: u32 id used to uniquly identify items
                date_created: String date the item was created
            Return:
        */
        pub fn new(item_name: String, item_id: u32, date_created: String) -> TodoItem{
           
            TodoItem{
                item_name, 
                item_id,
                date_created, 
                date_complete: String::from(""), 
                complete: false,
            }
        }
        /*
            Creates an item that exists in the database
            Prams:
                item_name: String name for the item
                item_id: u32 id used to uniquly identify items
                date_created: String date the item was created
                date_complete: String date that the item was completed
                complete: bool used to mark an item complete
            Return:
        */
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
        }
        /*
            Return: return a clone of the item name
        */
        pub fn get_item_name(&self) ->String{
            return self.item_name.clone();
        }
        /*
            Return: returns a clone of the data created
        */
        pub fn get_date_created(&self) ->String{
            return self.date_created.clone();

        }
        /*
            Return: returns a clone of the data complete
        */
        pub fn get_date_complete(&self)-> String{
            return self.date_complete.clone();

        }
        /*
            Prams: 
                date: String the date the item was completed
            Return: none
        */
        pub fn set_date_complete(&mut self, date: String){
            self.date_complete = date;
        }
        /*
            Return: bool item complete status
        */
        pub fn get_complete(&self) -> bool{
            return self.complete;
        }
        /*
            Return: u32 the item id
        */
        pub fn get_item_id(&self)->u32{
            return self.item_id;
        }

    }
}