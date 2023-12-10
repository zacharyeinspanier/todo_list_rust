

pub mod app_state{

    use crate::todo::todo::TodoList;
    use crate::user::user::User;
    use crate::database::database::TodoDatabase;
    use rand;

    /*
        This enum sets the input the user selects

        Members:
            AddList: User can add a new list
            AddItem: User can add a new item
            Default: User cannot add list or item
    */
    #[derive(PartialEq)]
    enum InputBox{
        AddList,
        AddItem,
        Default,
    }

    /*
        This enum sets the state for the app

        Members:
            CaptureInput: user can enter input to add list or item
            Navigate: user can navigate all lists and items
            Default: user can enter CaptureInput, Navigate, or exit the app
    */
    #[derive(PartialEq)]
    pub enum ActionState{
        CaptureInput,
        Navigate,
        Default,
    }
    /*
        This enum sets the list the users selects

        Members:
            List: user can navigate all list
            Items: user can naviage the curret list itmes
            Default: user cannot naviage items or lists
    */
    #[derive(PartialEq)]
    enum SelectedList{
        List,
        Items,
        Default,
    }

    /*
        This structue holds data about our state for the todo app

        Members:
            user: User, the user logged in
            database: TodoDatabase, the databse to add, remove, and update
            todo_lists: Vec<TodoList>, all of the user's todo lists
            input_list: String, store input for list name
            input_item: String, store input for item name
            list_index: usize, the list index that is currently selected 
            item_index: usize, the item index that is currently selected
            action_state: ActionState, action state for app
            input_box: InputBox, the selected input box
            selected_list: SelectedList, the selected list

    */
    pub struct State {
        pub user: User,
        pub database: TodoDatabase,
        pub todo_lists: Vec<TodoList>,
        pub input_list: String,
        pub input_item: String,
        pub list_index: usize,
        pub item_index: usize,
        pub action_state: ActionState,
        pub footer_meaage: String,
        input_box: InputBox,
        selected_list: SelectedList,

    }
    impl State{
        /*
            This method creates a new state struct

            Prams:
                user: User, the user logged in
                database: the database to user

            Returns: a state struct
        */
        pub fn new(user: User, database: TodoDatabase) -> State {
            // load user data
            let user_data: Vec<TodoList> = database.load_user_data(user.get_user_id());

            State {
                user,
                database,
                todo_lists: user_data,
                list_index: 0,
                item_index: 0,
                input_list: String::new(),
                input_item: String::new(),
                footer_meaage: String::from("Press 1 to enter input \nPress 2 to navigate\n Press q to exit app"),
                selected_list: SelectedList::Default,
                action_state: ActionState::Default,
                input_box: InputBox::Default,
            }
        }

        /*
            This method hanedls user pressing enter
        */
        pub fn add(&mut self){
            // If InputBox::AddList add_list()
            // If InputBox::AddItem add_item()
            match self.input_box{
                InputBox::AddList =>{
                    self.add_list();
                },
                InputBox::AddItem =>{
                    self.add_item();
                },
                _ => {},
            } 
        }

        /*
            This method adds a new list to the todo_lists
        */
        pub fn add_list(&mut self){

            // get list name
            let list_name: String = self.get_input();

            // empty string is not allowed
            if list_name != String::from(""){
                let mut list_id: u32;
                loop{
                    // generate a random list_id
                    list_id = rand::random::<u32>();
                    if list_id == u32::MAX {continue;}
                    // insert into database, err if the list_id exists
                    match self.database.insert_into_list(list_name.clone(), list_id, self.user.get_user_id()){
                        Ok(_res)=>{break;},
                        Err(err)=>{
                            match err.sqlite_error_code().unwrap(){
                                rusqlite::ErrorCode::ConstraintViolation => { continue;}
                                _ =>{panic!("{}", err)}
                            }
                        },
                    };
                }
                // add the new list to todo_lists
                self.todo_lists.push(TodoList::new(list_name, list_id));  
            }
        }

        /*
            This method adds a new item to the selected list
        */
        pub fn add_item(&mut self){

            //get item name
            let item_name: String = self.get_input();

            // empty string is not allowed
            if item_name != String::from(""){
                let mut item_id: u32;
                // get the current list_id
                let list_id = self.todo_lists[self.list_index].get_list_id();

                loop{
                    // generate random item_id 
                    item_id = rand::random::<u32>();
                    if item_id == u32::MAX {continue;}
                    // insert item into databse, err if the item_id already exists
                    match self.database.insert_into_items(item_name.clone(), item_id, list_id, 0){
                        Ok(_res)=>{break;},
                        Err(err) =>{
                            match err.sqlite_error_code().unwrap(){
                                rusqlite::ErrorCode::ConstraintViolation => { continue;}
                                _ =>{panic!("{}", err)}
                            }
                        },
                    };
                }
                // add the item to the current list
                self.todo_lists[self.list_index].add(item_name, item_id);
            }
        }

        /*
            This method handels user pressing delete/backspace
        */
        pub fn delete(&mut self){
            // if SelectedList::List delete_list()
            // if SelectedList::Items delete_item()
            match self.selected_list{
                SelectedList::List=>{
                    self.delete_list();
                },
                SelectedList::Items=>{
                    self.delete_item();
                },
                _ =>{},
            }
        }
        
        /*
            This method handles deleting a list from todo_lists
        */
        fn delete_list(&mut self){

            let n: usize = self.todo_lists.len();

            // Must be at lease 1 item in todo_lists
            if n > 0{
                let list_id = self.todo_lists[self.list_index].get_list_id();
                let user_id = self.user.get_user_id();
                // delete all list items from current list
                let remove_res = self.todo_lists[self.list_index].delete_list_items();
                // remove the list from todo_lists
                self.todo_lists.remove(self.list_index);

                // Remove list from databse
                if remove_res{
                    match self.database.remove_list(list_id, user_id){
                        Ok(()) =>{},
                        Err(err) =>{println!("{}", err)},
                    };
                }

                // set previous if last index in todo_lists removed
                if n-1 > 0 && n-1 == self.list_index{
                    self.previous_list_item();
                } 
            }
        }

        /*
            This method handles deleting a item from current list
        */
        fn delete_item(&mut self){

            let n: usize = self.todo_lists[self.list_index].get_list_len();
            // Must be at least One time in the list
            if n > 0{
                let list_id = self.todo_lists[self.list_index].get_list_id();
                let item_id = self.todo_lists[self.list_index].get_item_id(self.item_index);
                // remove item from list
                let remove_res = self.todo_lists[self.list_index].remove_index(self.item_index);

                // Remove item from database
                if remove_res{
                    match self.database.remove_item(item_id, list_id){
                        Ok(()) =>{},
                        Err(err) =>{println!("{}", err)},
                    };
                }

                // set previous if last index in list removed
                if n-1 > 0 && n-1 == self.item_index{
                    self.previous_list_item();
                } 
            }
        }

        /*
            This method marks an item complete
        */
        pub fn check_off(&mut self){
            // Only allowed on SelectedList::Items
            if self.selected_list == SelectedList::Items{

                // set item complete
                self.todo_lists[self.list_index].set_item_complete(self.item_index);

                let item_id = self.todo_lists[self.list_index].get_item_id(self.item_index);
                let list_id = self.todo_lists[self.list_index].get_list_id();

                // update item complete with date and time
                if self.todo_lists[self.list_index].get_item_complete_status(self.item_index){
                    match self.database.update_item(item_id, list_id, 1){
                        Ok(()) =>{},
                        Err(err) =>{println!("{}", err)},
                    };
                }
                else{
                    // item was marked not complete
                    match self.database.update_item(item_id, list_id, 0){
                        Ok(()) =>{},
                        Err(err) =>{println!("{}", err)},
                    };
                }
            }
        }

        /*
            This method gets the input from input boxes
        */
        pub fn get_input(&mut self) -> String{
            // if InputBox::AddList drain input_list
            // if InputBox::AddItem drain input_item
            match self.input_box{
                InputBox::AddList =>{return self.input_list.drain(..).collect();},
                InputBox::AddItem =>{return self.input_item.drain(..).collect();},
                _ => { return String::from("");},
            } 
        }

        /*
            This method sets the default state for the app
        */
        pub fn defalut_state(&mut self){
            self.selected_list = SelectedList::Default;
            self.input_box = InputBox::Default; 
            self.action_state =  ActionState::Default;
            self.input_list.drain(..);
            self.input_item.drain(..);
            self.footer_meaage = String::from("Press 1 to enter input. \nPress 2 to navigate.\n Press q to exit app.");
        }

        /*
           This method sets the capture input state for the app
        */
        pub fn capture_input_state(&mut self){
            self.selected_list = SelectedList::Default;
            self.input_box = InputBox::AddList; 
            self.action_state =  ActionState::CaptureInput;
            self.footer_meaage = String::from("Press right or left arrow key to choose input box. \nPress enter keys and press enter to add list or item. \n Press esc return to default.");
        }

        /*
            This method sets the navigate state for the app
        */
        pub fn navigate_state(&mut self){
            self.selected_list = SelectedList::List;
            self.input_box = InputBox::Default; 
            self.action_state =  ActionState::Navigate;
            self.footer_meaage = String::from("Press arrow keys to navigate list \nPress enter to cross off an item. \nPress backspace/delete to remove an item or list. \nPress esc return to default.");
        }

        /*
            This method handels the arrow key input from user
        */
        pub fn left_right_key(&mut self){
            match self.action_state{
                ActionState::Navigate=>{
                    match self.selected_list{
                        // toggle between Items and List
                        SelectedList::List =>{self.selected_list = SelectedList::Items},
                        SelectedList::Items => {self.selected_list = SelectedList::List},
                        _ =>{},
                    }
                },
                ActionState::CaptureInput=>{
                    match self.input_box{
                        // toggle between AddItem and AddList
                        InputBox::AddList =>{self.input_box = InputBox::AddItem},
                        InputBox::AddItem=> {self.input_box = InputBox::AddList},
                        _ =>{},
                    }
                },
                _ =>{},
            }
        }

        /*
            This method handels adding input from the user

            Prams
                c: char that user enters
        */
        pub fn add_input(&mut self, c: char){
            // if InputBox::AddList push to input_list
            // if InputBox::AddItem push to input_item
            match self.input_box{
                InputBox::AddList =>{self.input_list.push(c);},
                InputBox::AddItem =>{self.input_item.push(c);},
                _ =>{},
            }
        }

        /*
            This method handesl delete/backspace when input box is selected
        */
        pub fn remove_input(&mut self){
            // if InputBox::AddList then pop from input_list
            // if InputBox::AddItem then pop from input_item
            match self.input_box{
                InputBox::AddList =>{self.input_list.pop();},
                InputBox::AddItem =>{self.input_item.pop();},
                _ =>{},
            } 
        }

        /*
            This method is used to check if the input box is InputBox::AddList

            Returns: true if InputBox::AddList is selected, false otherwise
        */
        pub fn input_box_list(&self) -> bool{
            if self.action_state == ActionState::CaptureInput && self.input_box == InputBox::AddList{
                return true;
            }
            return false;
        }

        /*
            This method is used to check if the input box is InputBox::AddItem

            Returns: true if InputBox::AddItem is selected, false otherwise
        */
        pub fn input_box_item(&self) -> bool{

            if self.action_state == ActionState::CaptureInput && self.input_box == InputBox::AddItem{
                return true;
            }
            return false;
        }

        /*
            This method is used to check if the SelectedList is SelectedList::List

            Returns: true if SelectedList::List is selected, false otherwise
        */
        pub fn list_selected(&self) -> bool{
            if self.action_state == ActionState::Navigate && self.selected_list == SelectedList::List{
                return true;
            }
            return false;
        }

        /*
            This method is used to check if the SelectedList is SelectedList::Items

            Returns: true if SelectedList::Items is selected, false otherwise
        */
        pub fn item_selected(&self) -> bool{
            if self.action_state == ActionState::Navigate && self.selected_list == SelectedList::Items{
                return true;
            }
            return false;
        }

        /*
            This method sets the item_index or list_index to the one below the current
        */
        pub fn next_list_item(&mut self){
            match self.selected_list{
                SelectedList::List =>{
                    if self.list_index < self.todo_lists.len()-1{
                        self.list_index +=1;
                    }
                    else{
                        self.list_index = 0;
                    }
                },
                SelectedList::Items =>{
                    if self.item_index < self.todo_lists[self.list_index].get_list_len() - 1{
                        self.item_index += 1;
                    }
                    else{
                        self.item_index = 0;
                    }
                },
                _ => {},

            }
        }

        /*
            This method sets the item_index or list_index to the one above the current
        */
        pub fn previous_list_item(&mut self){
            match self.selected_list{
                SelectedList::List =>{
                    if self.list_index > 0{
                        self.list_index -= 1;
                    }
                    else{
                        self.list_index = self.todo_lists.len() - 1;
                    }
                },
                SelectedList::Items =>{
                    if self.item_index > 0{
                        self.item_index -= 1;
                    }
                    else{
                        self.item_index = self.todo_lists[self.list_index].get_list_len() - 1;
                    }
                    
                },
                _ => {},
            }
        }

        /*
            This method get the username of the current user

            Returns: String, username
        */
        pub fn get_username(&self)->String{
            return self.user.get_username();
        }

        /*
            This method get the current action_state

            Returns: String, actions_state
        */
        pub fn get_mode(&self)->String{
            match self.action_state{
                ActionState::CaptureInput=>{
                    return String::from("Input");
                },
                ActionState::Navigate =>{
                    return String::from("Navigation");
                },
                ActionState::Default=>{
                    return String::from("Default");
                },
            }
        }

        /*
           This method returns the name of the current list

           Returns: String, current list name
        */
        pub fn get_list_name(&self) -> String{
            if self.todo_lists.len() > 0{
                return self.todo_lists[self.list_index].get_name();
            }
            else{
                return String::from("No Active Lists");
            }
            
        }

        /*
            This method handles the user pressing enter
        */
        pub fn handel_enter(&mut self){
            // if SelectedList::List then capture_input_state()
            // if SelectedList::Items check_off()
            match self.selected_list{
                SelectedList::List => {self.capture_input_state();},
                SelectedList::Items => {self.check_off();},
                _ => {},
            }
        }
    
    }
}