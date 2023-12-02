use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame,
};
use unicode_width::UnicodeWidthStr;
use crate::app_state::app_state::State;

/*
    This enum is used to draw the lists and list items

    Members:
        AllLists: draw all lists
        ListItems: draw list itmes
*/
enum DrawList{
    AllLists,
    ListItems,
}

/*
    This function renders the todo app

    Prams: 
        f: Fram the terminal fram to draw in
        state: State, the state of the app
*/
pub fn render_ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let size = f.size();

    // split fram into 3 chunks
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


    // set background
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    // draw the header
    draw_header(f, state, chunks[0]);
    
    // draw list
    draw_list_display(f, state, chunks[1]);
    
    // draw footer
    let footer = draw_footer(state);
    f.render_widget(footer, chunks[2]);
}

/*
    This function draws the header

    Prams: 
        f: Fram the terminal fram to draw in
        state: State, the state of the app
        size: Rect, space where header will be drawn
*/
fn draw_header<B: Backend>(f: &mut Frame<B>, state: &mut State, size: Rect){

    // split size into chucks
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref())
        .split(size);
        // draw username
        let username = draw_username(state);
        f.render_widget(username, header_chunks[0]);
        // draw mode
        let mode = draw_mode(state);
        f.render_widget( mode, header_chunks[1]);
        // draw list name
        let list_name = draw_list_name(state);
        f.render_widget( list_name, header_chunks[2]);
}

/*
    This function creates a Paragraph where the username is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with user name
*/
fn draw_username(state: &State) -> Paragraph{

    return Paragraph::new(state.get_username())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Current User"));
}

/*
    This function creates a Paragraph where the app mode is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with state mode
*/
fn draw_mode(state: &State) -> Paragraph{
    return Paragraph::new(state.get_mode())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Current Mode"));
}

/*
    This function creates a Paragraph where the current list name is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with current list name
*/
fn draw_list_name(state: &State) ->Paragraph{
    return Paragraph::new(state.get_list_name())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Selected List"));
}

/*
    This function draws all todo lists and todo items

    Prams: 
        f: Fram the terminal fram to draw in
        state: State, the state of the app
        size: Rect, space where lists will be drawn
*/
fn draw_list_display<B: Backend>(f: &mut Frame<B>, state: &mut State, size: Rect){

    // split size into 2 chunks
    let list_input_chunks= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref())
        .split(size);
        // displays the lists and input box
        let all_list_chunk:Vec<Rect> = draw_list_input_box(f,  state, list_input_chunks[0], DrawList::AllLists);
        // displays the lists and input box
        let item_list_chunk: Vec<Rect> = draw_list_input_box(f,  state, list_input_chunks[1], DrawList::ListItems);

        // set curser
        if state.input_box_list(){
            f.set_cursor(all_list_chunk[1].x + state.input_list.width() as u16 + 1, all_list_chunk[1].y+1);
        }
        else if state.input_box_item(){
            f.set_cursor(item_list_chunk[1].x + state.input_item.width() as u16 + 1, item_list_chunk[1].y+1);
        }
}

/*
    This function draws the all todo lists and a list input box

    Prams: 
        f: Fram the terminal fram to draw in
        state: State, the state of the app
        size: Rect, space where list
        draw_type: DrawList, the type of list to draw(AllLists, ListItems)

    Returns: Vec<Rect> containing the chunks for list and input box
*/
fn draw_list_input_box<B: Backend>(f: &mut Frame<B>, state: &mut State, size: Rect, draw_type: DrawList) -> Vec<Rect>{

    // split size into two chucks
    let list_input_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref())
        .split(size);

    // draw either AllLists or ListItems
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

/*
    This function creates a Paragraph where the list input is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with list input
*/
fn draw_input_list_name(state: &State) -> Paragraph{
    return Paragraph::new(state.input_list.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("List Input"));
}

/*
    This function creates a Paragraph where the item input is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with item ist input
*/
fn draw_input_item_name(state: &State) -> Paragraph{
    return Paragraph::new(state.input_item.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Item Input"));
}

/*
    This function creates a list of the curret list todo items

    Prams: 
        state: State, the state of the app
    
    Returns: List of curret list todo items
*/
fn draw_list_todo_items(state: &State) -> List{

    let items: Vec<ListItem>;

    // empty list
    if state.todo_lists.len() == 0{
        items = Vec::new();
    }
    else{
        // creates ListItem for each item
        items = state
            .todo_lists[state.list_index]
            .list
            .iter()
            .enumerate()    
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.get_item_name())))];
                // item is selected
                if state.item_selected() && i == state.item_index{
                    if m.get_complete(){
                        // item is marked complete
                        ListItem::new(content).style(Style::default().fg(Color::Red).bg(Color::Blue).add_modifier(Modifier::CROSSED_OUT))
                    }
                    else{
                        ListItem::new(content).style(Style::default().fg(Color::Red).bg(Color::Blue))
                    }
                }
                else if m.get_complete(){
                    // item is marked complete
                    ListItem::new(content).style(Style::default().add_modifier(Modifier::CROSSED_OUT))
                }
                else{
                    ListItem::new(content)
                }
                
            })
            .collect();
    }

    // Create the List 
    return List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List Items"))
        .style(
            Style::default()
        );

}

/*
    This function creates a list of all the todo lists

    Prams: 
        state: State, the state of the app
    
    Returns: List of all the todo lists
*/
fn draw_list_todo_lists(state: &State) -> List{

    // creates ListItem for each list
    let all_lists: Vec<ListItem>  = state
        .todo_lists
        .iter()
        .enumerate()    
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.get_name())))];
            // item is selected
            if state.list_selected() && i == state.list_index{
                ListItem::new(content).style(Style::default().fg(Color::Red).bg(Color::Blue),)
            }
            else{
                ListItem::new(content)
            }
        })
    .collect();

    // Create the List
    return List::new(all_lists)
    .block(Block::default().borders(Borders::ALL).title("All Lists"))
    .style(
        Style::default()
    );

}

/*
    This function creates a Paragraph where messages to the user is drawn

    Prams: 
        state: State, the state of the app
    
    Returns: Paragraph, with messages to the user
*/
fn draw_footer(state: &State)-> Paragraph{

    let mut message_text = Text::from(Spans::from(state.footer_meaage.as_ref()));
    message_text.patch_style(Style::default());

    return Paragraph::new(message_text)
    .block(Block::default().borders(Borders::ALL).title("Message"));
}