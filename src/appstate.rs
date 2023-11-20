

pub mod appstate{
    use crate::todo::todo::TodoList;
    use crate::user::user::User;
    use crate::database::database::TodoDatabase;
    use rand;

    #[derive(PartialEq)]
    enum InputBox{
        AddList,
        AddItem,
        Default,
    }
    #[derive(PartialEq)]
    pub enum ActionState{
        CaptureInput,
        Navigate,
        Default,
    }
    #[derive(PartialEq)]
    enum SelectedList{
        List,
        Items,
        Default,
    }

    pub struct State {
        pub user: User,
        pub database: TodoDatabase,
        pub todo_lists: Vec<TodoList>,
        pub index: usize,
        pub input_list: String,
        pub input_item: String,
        pub list_index: usize,
        pub item_index: usize,
        pub action_state: ActionState,
        input_box: InputBox,
        selected_list: SelectedList,

    }
    impl State{
        pub fn new(tutorial: bool, user: User, database: TodoDatabase) -> State {
            // load user data
            let mut user_data: Vec<TodoList> = database.load_user_data(user.get_user_id());
            State {
                user,
                database,
                todo_lists: user_data,
                index: 0,
                list_index: 0,
                item_index: 0,
                input_list: String::new(),
                input_item: String::new(),
                selected_list: SelectedList::Default,
                action_state: ActionState::Default,
                input_box: InputBox::Default,
            }
        }

        pub fn add(&mut self){
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

        pub fn add_list(&mut self){
            let list_name: String = self.get_input();
            if list_name != String::from(""){
                let mut list_id: u32 = 0;
                loop{
                    list_id = rand::random::<u32>();
                    match self.database.insert_into_list(list_name.clone(), list_id, self.user.get_user_id()){
                        Ok(res)=>{break;},
                        Err(err)=>{continue;},
                    };
                }
                self.todo_lists.push(TodoList::new(list_name, list_id));
                
            }
        }
        pub fn add_item(&mut self){
            let item_name: String = self.get_input();
            if item_name != String::from(""){
                let mut item_id: u32 = 0;
                let list_id = self.todo_lists[self.index].get_list_id();
                let date_created = chrono::offset::Local::now().to_string();

                loop{
                    item_id = rand::random::<u32>();
                    match self.database.insert_into_items(item_name.clone(), item_id, list_id, date_created.clone(), String::from(""), 0){
                        Ok(res)=>{break;},
                        Err(err) =>{continue;}, // could have error message
                    };

                }
                self.todo_lists[self.index].add(item_name, item_id, date_created);
            }
        }

        pub fn delete(&mut self){
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
        
        fn delete_list(&mut self){

            let n: usize = self.todo_lists.len();
            // only one list
            if n > 0{
                let list_id = self.todo_lists[self.list_index].get_list_id();
                let remove_res = self.todo_lists[self.list_index].delete_list_items();
                self.todo_lists.remove(self.list_index);

                if remove_res{
                    match self.database.remove_list(list_id, self.user.get_user_id()){
                        Ok(()) =>{},
                        Err(err) =>{println!("{}", err)},
                    };
                }

                 // if the item removed is at the smae index as selected
                if self.index >= self.list_index {
                    self.previous_tab()
                }
                // set previous if last index in list removed
                if n-1 > 0 && n-1 == self.list_index{
                    self.previous_list_item();
                } 
               
            }

        }
        fn delete_item(&mut self){
            let n: usize = self.todo_lists[self.index].get_list_len();

            if n > 0{
                let item_id = self.todo_lists[self.index].list[self.item_index].get_item_id();
                let remove_res = self.todo_lists[self.index].remove_index(self.item_index);

                if remove_res{
                    match self.database.remove_item(item_id, self.todo_lists[self.index].get_list_id()){
                        Ok(()) =>{},
                        Err(err) =>{},
                    };
                }
                // set previous if last index in list removed
                if n-1 > 0 && n-1 == self.item_index{
                    self.previous_list_item();
                } 
            }
             // if successful remove from db 
        }

        pub fn check_off(&mut self){
            if self.selected_list == SelectedList::Items{
                self.todo_lists[self.index].item_complete_index(self.item_index);
            }
            // update db
        }



        pub fn get_input(&mut self) -> String{
            match self.input_box{
                InputBox::AddList =>{return self.input_list.drain(..).collect();},
                InputBox::AddItem =>{return self.input_item.drain(..).collect();},
                _ => { return String::from("");},
            } 
        }

       

        pub fn defalut_state(&mut self){
            self.selected_list = SelectedList::Default;
            self.input_box = InputBox::Default; 
            self.action_state =  ActionState::Default;
            self.input_list.drain(..);
            self.input_item.drain(..);
        }
        pub fn capture_input_state(&mut self){
            self.selected_list = SelectedList::Default;
            self.input_box = InputBox::AddList; 
            self.action_state =  ActionState::CaptureInput;

        }
        pub fn navigate_state(&mut self){
            self.selected_list = SelectedList::List;
            self.input_box = InputBox::Default; 
            self.action_state =  ActionState::Navigate;

        }

        pub fn left_right_key(&mut self){
            // Toggle between list and times
            match self.action_state{
                ActionState::Navigate=>{
                    match self.selected_list{
                        SelectedList::List =>{self.selected_list = SelectedList::Items},
                        SelectedList::Items => {self.selected_list = SelectedList::List},
                        _ =>{},
                    }
                },
                ActionState::CaptureInput=>{
                    match self.input_box{
                        InputBox::AddList =>{self.input_box = InputBox::AddItem},
                        InputBox::AddItem=> {self.input_box = InputBox::AddList},
                        _ =>{},
                    }
                },
                _ =>{},
            }
        }


        pub fn add_input(&mut self, c: char){
            match self.input_box{
                InputBox::AddList =>{self.input_list.push(c);},
                InputBox::AddItem =>{self.input_item.push(c);},
                _ =>{},
            } 

        }
        pub fn remove_input(&mut self){
            match self.input_box{
                InputBox::AddList =>{self.input_list.pop();},
                InputBox::AddItem =>{self.input_item.pop();},
                _ =>{},
            } 

        }

        pub fn input_box_list(&self) -> bool{
            if self.action_state == ActionState::CaptureInput && self.input_box == InputBox::AddList{
                return true;
            }
            return false;
        }
        pub fn input_box_item(&self) -> bool{

            if self.action_state == ActionState::CaptureInput && self.input_box == InputBox::AddItem{
                return true;
            }
            return false;
        }

        pub fn list_selected(&self) -> bool{
            if self.action_state == ActionState::Navigate && self.selected_list == SelectedList::List{
                return true;
            }
            return false;
        }
        pub fn item_selected(&self) -> bool{
            if self.action_state == ActionState::Navigate && self.selected_list == SelectedList::Items{
                return true;
            }
            return false;
        }

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
                    if self.item_index < self.todo_lists[self.index].get_list_len() - 1{
                        self.item_index += 1;
                    }
                    else{
                        self.item_index = 0;
                    }
                },
                _ => {},

            }

        }
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
                        self.item_index = self.todo_lists[self.index].get_list_len() - 1;
                    }
                    
                },
                _ => {},

            }
        }

        pub fn next_tab(&mut self) {

            if self.index < self.todo_lists.len()-1{
                self.index +=1;
            }
            else if self.todo_lists.len() == 0{
                self.index = 0;

            }
            else{
                self.index = 0;
            }
        }
    
        pub fn previous_tab(&mut self) {
            if self.index > 0 {
                self.index -= 1;
            }
            else if self.todo_lists.len() == 0{
                self.index = 0;

            }
             else {
                self.index = self.todo_lists.len() - 1;
            }
        }        
    }
}