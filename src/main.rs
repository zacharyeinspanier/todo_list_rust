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
use crate::appstate::appstate::{State, InputMode, InputBox};

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
        terminal.draw(|f| render_ui(f, &state))?;

        // key code will be use to set the InputBox

        if let Event::Key(key) = event::read()? {
            match state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {return Ok(())},
                    KeyCode::Right => {state.next_tab();},
                    KeyCode::Left => {state.previous_tab();},
                    KeyCode::Char('e') => {
                        state.input_box = InputBox::AddList;
                        state.input_mode = InputMode::Editing;
                    },
                    KeyCode::Char('i') => {
                        state.input_box = InputBox::AddItem;
                        state.input_mode = InputMode::Editing
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {state.input_mode = InputMode::Normal;}
                    KeyCode::Char(c) => {
                        match state.input_box{
                            InputBox::AddList =>{state.input_list.push(c);},
                            InputBox::AddItem =>{state.input_item.push(c);},
                        } 
                    }
                    KeyCode::Backspace => {
                        match state.input_box{
                            InputBox::AddList =>{state.input_list.pop();},
                            InputBox::AddItem =>{state.input_item.pop();},
                        } 
                    }
                    KeyCode::Enter => {
                        match state.input_box{
                            InputBox::AddList =>{state.add_list();},
                            InputBox::AddItem =>{state.add_item();},
                        } 
                    }
                    _ => {}

                },
            }
        }
    }
}

