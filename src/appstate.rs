

pub mod appstate{
    use crate::todo::todo::{TodoList, TodoItem};

    // titles: list of all todoList names
    // index: current tab
    // input: string for input box
    // input mode: input mode: nomal, editing
    // todoLists: vector of todo Lists
    // list state: state of list

    // on every enter:
        // create new list and add it to the todoLists

    pub enum InputMode {
        Normal,
        Editing,
    }

    pub struct State<'a> {
        pub titles: Vec<&'a str>, // list of todoLists
        pub index: usize,
        pub input: String,
        pub input_mode: InputMode,
        pub all_input: Vec<String>,
        pub todo_lists: Vec<TodoList>,

    }
    //TODO add dictionary to hold todolists
    impl<'a> State<'a> {
        pub fn new() -> State<'a> {
            State {
                titles: vec!["home", "Tab1", "Tab2", "Tab3"],
                index: 0,
                input: String::new(),
                input_mode: InputMode::Normal,
                all_input: Vec::new(),
                todo_lists: Vec::new(),
            }
        }
    
        pub fn next(&mut self) {
            self.index = (self.index + 1) % self.titles.len();
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