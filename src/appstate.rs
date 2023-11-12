

pub mod appstate{
    use crate::todo::todo::{TodoList, TodoItem};


    // use enume for create list and add item
    // home page will be create list
    // all other tabs will be add item

    // add a list:
        // create list and add it to the todo_lists
        // take name and add it to tabs
    // add item:
        // append to current list
        // state.todolist[index].add("new item")

    pub enum TabType{
        Home,
        ListSelected,
    }

    pub enum InputMode {
        Normal,
        Editing,
    }

    pub struct State {
        pub titles: Vec<String>,
        pub index: usize,
        pub input: String,
        pub input_mode: InputMode,
        pub todo_lists: Vec<TodoList>,
        pub tab_type: TabType,

    }
    //TODO add dictionary to hold todolists
    impl State{
        pub fn new() -> State {
            State {
                titles: vec!["home".to_string()],
                index: 0,
                input: String::new(),
                input_mode: InputMode::Normal,
                todo_lists: Vec::new(),
                tab_type: TabType::Home,
            }
        }

        pub fn update_titles(&mut self){
            let list_name: String = self.input.drain(..).collect();
            if !self.titles.contains(&list_name){

                self.todo_lists.push(TodoList::new(String::from(list_name.clone())));
                while self.titles.len() > 1 as usize{
                    self.titles.pop();
                }
                let mut new : Vec<String> = self.todo_lists.iter().map(|x| x.name.clone()).collect();
                self.titles.append(&mut new);
            }
              
        }
    
        pub fn next(&mut self) {
            if self.index < self.titles.len()-1{
                self.index +=1;
            }
            else{
                self.index = 0;
            }
        }
    
        pub fn previous(&mut self) {
            if self.index > 0 {
                self.index -= 1;
            } else {
                self.index = self.titles.len() - 1;
            }
        }
    }
}