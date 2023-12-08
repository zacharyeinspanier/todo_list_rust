pub mod todo_item{


    /*
        This structue hold data about TodoItem

        Members:
            item_name: String, item name
            item_id: u32, unique identifier for item
            complete: bool, complete status true = complete false = not complete


    */
    pub struct TodoItem{
        item_name: String,
        item_id: u32,
        complete: bool,
    }
    impl TodoItem{

        /*
            Create new todo item item
            Prams:
                item_name: String name for the item
                item_id: u32 id used to uniquly identify items
            Return:
        */
        pub fn new(item_name: String, item_id: u32) -> TodoItem{
           
            TodoItem{
                item_name, 
                item_id, 
                complete: false,
            }
        }
        /*
            Creates an item that exists in the database
            Prams:
                item_name: String name for the item
                item_id: u32 id used to uniquly identify items
                complete: bool used to mark an item complete
            Return:
        */
        pub fn new_from_load(item_name: String, item_id: u32, complete: bool) -> TodoItem{ 
            TodoItem{
                item_name, 
                item_id,
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