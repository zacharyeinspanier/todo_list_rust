mod render;
mod todo;
mod todo_item;
mod appstate;
mod database;
mod user;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crate::render::render_ui;
use crate::appstate::appstate::{State, ActionState};
use crate::database::database::{TodoDatabase, QueryUser};
use rusqlite::{params, Connection, Result};
use crate::user::user::User;





fn main() -> Result<(), Box<dyn Error>> {
    // create database_manager
    // user login
        // create user
    // database_manager.load_data
    // create state(user, data, database_manager)
    // start app

    //option 2
    // load data is in main.rs



    let connection = Connection::open("database/data.db")?;
    let our_db = TodoDatabase::new(String::from("ddatabase/data.db"), connection);
    
    // build 
    our_db.build_db()?;

    // user
    let mut user_query: Vec<QueryUser> = match our_db.get_user_id("user1", "1235"){
        Ok(res) =>{res},
        Err(err) => {panic!("{}", err);}
    };
  
    
    let current_user = User::user_login(user_query[0].user_id, user_query[0].username.clone(), user_query[0].password.clone());

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let state = State::new(false, current_user, our_db);
    let res = run_app(&mut terminal, state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: State) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, &mut state))?;

        // key code will be use to set the InputBox

        if let Event::Key(key) = event::read()? {
           
            match state.action_state{
                ActionState::Default => match key.code {
                    KeyCode::Right => {state.next_tab();},
                    KeyCode::Left => {state.previous_tab();},
                    KeyCode::Esc => {state.defalut_state();},
                    KeyCode::Char('1') => {state.capture_input_state()},
                    KeyCode::Char('2') => {state.navigate_state()},
                    KeyCode::Char('q') => {return Ok(())},
                    _ => {},
                },
                ActionState::CaptureInput => match key.code {
                    KeyCode::Esc => {state.defalut_state();},
                    KeyCode::Char(c) => {state.add_input(c);},
                    KeyCode::Backspace => {state.remove_input();},
                    KeyCode::Enter => {state.add();}
                    KeyCode::Left => {state.left_right_key();},
                    KeyCode::Right => {state.left_right_key();},
                    _ => {},
                },
                ActionState::Navigate => match key.code {
                    KeyCode::Esc => {state.defalut_state();},
                    KeyCode::Left => {state.left_right_key();},
                    KeyCode::Right => {state.left_right_key();},
                    KeyCode::Up => {state.previous_list_item();},
                    KeyCode::Down => {state.next_list_item();},
                    KeyCode::Backspace => {state.delete();},
                    KeyCode::Enter => {state.check_off();}
                    _ => {},
                },
            }
        }
    }
}


