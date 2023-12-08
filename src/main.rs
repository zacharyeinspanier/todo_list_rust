pub mod render;
pub mod app_state;
mod database;
mod user;
mod todo;
mod todo_item;
mod user_authentication;
mod render_authenitcation;

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
use render_authenitcation::render_user_authentication;
use database::database::TodoDatabase;
use rusqlite::{Result};
use user_authentication::user_authentication::{Authentication, AuthenticationState};
use crate::user::user::User;

use crate::app_state::app_state::{State, ActionState};



/*
    This function is the main function and runs the app

    Returns: Result<Ok, Box<err>
        Ok(): app runs successfully
        Box<err>: Error with terminal
*/
fn main() -> Result<(), Box<dyn Error>> {   
   
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout(); // output to termain
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?; //create termainl
    terminal.clear()?;

    // create database for  Authentication
    let our_db = TodoDatabase::new(String::from("database/data.db"));
    let authen = Authentication::new(our_db);
    // runs the user authentication, returns Result<Option<User>, Err>
    let res: Option<User> = match run_user_authentication(&mut terminal, authen){
        Ok(option) => {option},
        Err(err) => {panic!("there was an error {}", err)},
    };

    
    if !res.is_none(){
        let user: User = res.unwrap();
        let db = TodoDatabase::new(String::from("database/data.db"));
        // create app and run it

        let state = State::new(user, db);
        let res_app = run_app(&mut terminal, state);

        match res_app{
            Err(err) =>{ println!("{:?}", err);},
        _ =>{},
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/*
    This function runs user authentication

    Prams:
        terminal: the terminal to display app in
        user_authenticate: Authentication state

    Returns: Result<Ok(Option<User>), Err>
        Ok(Option<User>): app ran successfully and option was returned
        Err: app failed and there was an error 
*/
fn run_user_authentication<B: Backend>(terminal: &mut Terminal<B>, mut user_authenticate: Authentication)-> io::Result<Option<User>>{
    // continually render app
    loop {
        // draw widgets
        terminal.draw(|f| render_user_authentication(f, &mut user_authenticate))?;

        // handel input keys, calling user_authenticate methods
        if let Event::Key(key) = event::read()? {
            match user_authenticate.authentication_state{
                AuthenticationState::Default => match key.code {
                    KeyCode::Char('1') => {user_authenticate.user_input();},
                    KeyCode::Char('q') => {return Ok(user_authenticate.get_user())},
                    _ =>{},
                },
                AuthenticationState::UserInput => match key.code {
                    KeyCode::Up => {user_authenticate.previous_index();},
                    KeyCode::Down => {user_authenticate.next_index();},
                    KeyCode::Enter => {user_authenticate.process_enter();},
                    KeyCode::Char(c) => {user_authenticate.add_input(c)},
                    KeyCode::Backspace => {user_authenticate.remove_input()},
                    KeyCode::Esc => {user_authenticate.default();},
                    _ =>{},
                },
                AuthenticationState::LoggedIn => {return Ok(user_authenticate.get_user())},
            }
        }
    }
}

/*
    This function runs todo app

    Prams:
        terminal: the terminal to display app in
        state: State for the todo app

    Returns: Result<Ok(Option<User>), Err>
        Ok(): app ran successfully
        Err: app failed and there was an error 
*/
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: State) -> io::Result<()> {
    // continually render app
    loop {
        // draw widgets
        terminal.draw(|f| render_ui(f, &mut state))?;

        // handel input keys, calling state methods
        if let Event::Key(key) = event::read()? {
            match state.action_state{
                ActionState::Default => match key.code {
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
                    KeyCode::Enter => {state.handel_enter();},
                    _ => {},
                },
            }
        }
    }
}