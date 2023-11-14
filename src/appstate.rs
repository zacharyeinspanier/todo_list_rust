

pub mod appstate{
    use crate::todo::todo::{TodoList, TodoItem};
    use tui::{
        widgets::{ListState},
        style::{Color, Modifier, Style},
        text::{Span, Spans, Text},
    };

    pub enum InputBox{
        AddList,
        AddItem,
    }

    pub enum InputMode {
        Normal,
        Editing,
    }

    pub struct State {
        pub todo_lists: Vec<TodoList>,
        pub index: usize,
        pub input_list: String,
        pub input_item: String,
        pub input_mode: InputMode,
        pub input_box: InputBox,
        pub list_state: ListState,
        pub instructions: String,

    }
    //TODO add dictionary to hold todolists
    impl State{
        pub fn new() -> State {
            State {
                todo_lists: vec![TodoList::new(String::from("test"))],
                index: 0,
                input_list: String::new(),
                input_item: String::new(),
                input_mode: InputMode::Normal,
                input_box: InputBox::AddList,
                list_state: ListState::default(),
                instructions: String::new(),
            }
        }

        pub fn add_list(&mut self){
            // drain input
            let list_name: String = self.input_list.drain(..).collect();
            // add list
            self.todo_lists.push(TodoList::new(list_name));
        }
        pub fn add_item(&mut self){
            // drain input
            let item_name: String = self.input_item.drain(..).collect();
            // add item
            self.todo_lists[self.index].add(item_name);
        }

        pub fn next_tab(&mut self) {

            if self.index < self.todo_lists.len()-1{
                self.index +=1;
            }
            else{
                self.index = 0;
            }
        }
    
        pub fn previous_tab(&mut self) {
            if self.index > 0 {
                self.index -= 1;
            } else {
                self.index = self.todo_lists.len() - 1;
            }
        }

        pub fn next_list_item(&mut self) -> i32{
            return 0;
        }

        pub fn previous_list_itme(&mut self) ->i32{
            return 0;
        }
    }
}