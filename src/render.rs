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
    widgets::{Block, Borders, Tabs, Paragraph, List, ListItem},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::appstate::appstate::{State, InputMode, TabType};


pub fn render_ui<B: Backend>(f: &mut Frame<B>, state: &State) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    
    let tabs = draw_tabs(state);
    f.render_widget(tabs, chunks[0]);

    let input = draw_input_box(state);
    f.render_widget(input, chunks[1]);

    match state.input_mode{
        InputMode::Normal=>{},
        InputMode::Editing=>{
            f.set_cursor(chunks[1].x + state.input.width() as u16 + 1, chunks[1].y+1);

        },
    }

    let list  = draw_list(state);
    f.render_widget(list, chunks[2]);
}

fn draw_tabs(state: &State) -> Tabs{
    let titles = state
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
        
        return Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(state.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    
}

fn draw_input_box(state: &State) -> Paragraph{
    return Paragraph::new(state.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    
}

fn draw_list(state: &State) -> List{
    // loop throug all todoLists
    // list.name

    // match here for the home page or the list type
    let items: Vec<ListItem> = match state.tab_type{
        TabType::Home=>{
            state
            .todo_lists
            .iter()
            .enumerate()    
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.name)))];
                ListItem::new(content)
            })
            .collect()
        },
        TabType::ListSelected=>{
            state
            .todo_lists[state.index-1]
            .list
            .iter()
            .enumerate()    
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.item_name)))];
                ListItem::new(content)
            })
            .collect()
        },
    };
    
    
    

    // set title 
    return List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

}

fn draw_footer()-> i32{
    return 0;
}