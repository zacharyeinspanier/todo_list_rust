use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Tabs, Paragraph, List, ListItem},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::appstate::appstate::State;
use tui::widgets::Wrap;

enum DrawList{
    AllLists,
    ListItems,
}


pub fn render_ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let size = f.size();
    //println!("{:?}", size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(65),
                Constraint::Percentage(20),

            ]
            .as_ref())
        .split(size);


    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    
    let tabs = draw_tabs(state);
    f.render_widget(tabs, chunks[0]);

    draw_list_display(f, state, chunks[1]);

    let footer = draw_footer();
    f.render_widget(footer, chunks[2]);
}

fn draw_tabs(state: &State) -> Tabs{
 
    let todo_names = state
        .todo_lists
        .iter()
        .map(|t| {
            Spans::from(Span::styled(&t.name, Style::default().fg(Color::Green)))
        })
        .collect();

        return Tabs::new(todo_names)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(state.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
}

fn draw_list_display<B: Backend>(f: &mut Frame<B>, state: &mut State, size: Rect){

    let list_input_chunks= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref())
        .split(size);
        
        let all_list_chunk:Vec<Rect> = draw_list_input_box(f,  state, list_input_chunks[0], DrawList::AllLists);
        let item_list_chunk: Vec<Rect> = draw_list_input_box(f,  state, list_input_chunks[1], DrawList::ListItems);


        if state.input_box_list(){
            f.set_cursor(all_list_chunk[1].x + state.input_list.width() as u16 + 1, all_list_chunk[1].y+1);
        }
        else if state.input_box_item(){
            f.set_cursor(item_list_chunk[1].x + state.input_item.width() as u16 + 1, item_list_chunk[1].y+1);
        }

}

fn draw_list_input_box<B: Backend>(f: &mut Frame<B>, state: &mut State, size: Rect, draw_type: DrawList) -> Vec<Rect>{
    let list_input_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref())
        .split(size);

    match draw_type{
        DrawList::AllLists=>{
            let list  = draw_list_todo_lists(state);
            f.render_widget(list, list_input_chunk[0]);

            let input = draw_input_list_name(state);
            f.render_widget(input, list_input_chunk[1]);
        },
        DrawList::ListItems=>{
            let list  = draw_list_todo_items(state);
            f.render_widget(list, list_input_chunk[0]);

            let input = draw_input_item_name(state);
            f.render_widget(input, list_input_chunk[1]);
        },
    }

    return list_input_chunk;
}

fn draw_input_list_name(state: &State) -> Paragraph{
    return Paragraph::new(state.input_list.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("List Input"));
}

fn draw_input_item_name(state: &State) -> Paragraph{
    return Paragraph::new(state.input_item.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Item Input"));
}

fn draw_list_todo_items(state: &State) -> List{

    let items: Vec<ListItem> = state
        .todo_lists[state.index]
        .list
        .iter()
        .enumerate()    
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.item_name)))];
            if state.item_selected() && i == state.item_index{
                ListItem::new(content).style(Style::default().fg(Color::Red).bg(Color::Blue))
            }
            else{
                ListItem::new(content)
            }
            
        })
        .collect();

    // set title 
    return List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List Items"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

}

fn draw_list_todo_lists(state: &State) -> List{

    let all_lists: Vec<ListItem>  = state
        .todo_lists
        .iter()
        .enumerate()    
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.name)))];
            if state.list_selected() && i == state.list_index{
                ListItem::new(content).style(Style::default().fg(Color::Red).bg(Color::Blue))
            }
            else{
                ListItem::new(content)
            }
            
            
        })
    .collect();

    // set title 
    return List::new(all_lists)
    .block(Block::default().borders(Borders::ALL).title("All Lists"))
    .highlight_style(
        Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

}

fn draw_footer()-> Paragraph<'static>{

    let mut text = Text::from(Spans::from(vec![
        Span::raw("Press e to enter name for list. Press ENTER to create the list."),
        Span::raw("Press i to enter item for your list. Press ENTER to add the item.\n"),
        Span::raw("Press esc to end input\n"),
        Span::raw("\nUse Arrow keys (left, right) to navigate tabs thus selecting list.\n"),
        Span::raw("To exit app "),
    ]));
    text.patch_style(Style::default());

    return Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .wrap(Wrap { trim: true });
}