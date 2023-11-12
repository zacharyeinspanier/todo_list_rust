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
use crate::appstate::appstate::{State, InputMode, TabType};

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

        if let Event::Key(key) = event::read()? {
            match state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {return Ok(())},
                    KeyCode::Right => {
                        state.next();
                        // update tab type
                        if state.index as i32 == 0{
                            state.tab_type = TabType::Home;
                        }
                        else{
                            state.tab_type = TabType::ListSelected;
                        }
                    },
                    KeyCode::Left => {
                        state.previous();
                        // update tab type
                        if state.index as i32 == 0{
                            state.tab_type = TabType::Home;
                        }
                        else{
                            state.tab_type = TabType::ListSelected;
                        }
                    },
                    KeyCode::Char('e') => {state.input_mode = InputMode::Editing},
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {state.input_mode = InputMode::Normal;}
                    KeyCode::Char(c) => {state.input.push(c);}
                    KeyCode::Backspace => {state.input.pop();}
                    KeyCode::Enter => {
                        // match on tab type
                        match state.tab_type{
                            TabType::Home =>{
                                state.update_titles()
                            },
                            TabType::ListSelected =>{
                                // new itme
                                let item_name:String = state.input.drain(..).collect();
                                state.todo_lists[state.index-1].add(item_name);
                            },
                        }
                        
                    }
                    _ => {}

                },
            }
        }
    }
}

