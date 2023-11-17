use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

mod render;
use crate::render::render_ui;

mod appstate;
use crate::appstate::appstate::{State, ActionState};

mod todo;
use crate::todo::todo::{TodoList, TodoItem};


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let state = State::new();
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
                    KeyCode::Char('1') => {state.capture_input_State()},
                    KeyCode::Char('2') => {state.navigate_State()},
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
                    //KeyCode::Backspace => {},
                    //KeyCode::Enter => {}
                    _ => {},
                },
            }
        }
    }
}



//Backspace
//Enter
//Delete
//Tab

