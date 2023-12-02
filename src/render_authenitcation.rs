use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Tabs, Paragraph, List, ListItem},
    Frame,
};
use crate::user_authentication::user_authentication::{Authentication, AuthenticationState, SelectedChunk};

/*
    This function renders the user authentication login page

    Prams:
        f: Frame useed to display widgets
        user_authenticate: Authenticationt holding the state of this page
*/
pub fn render_user_authentication<B: Backend>(f: &mut Frame<B>, user_authenticate: &mut Authentication) {

    let size = f.size();
    // separate fram into chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref())
        .split(size);

    // set background
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    // username input box
    let username = Paragraph::new(user_authenticate.username_input.as_ref())
    .style(Style::default())
    .block(Block::default().borders(Borders::ALL).title("username"));
    f.render_widget(username, chunks[0]);

    // password input box
    let password = Paragraph::new(user_authenticate.password_input.as_ref())
    .style(Style::default())
    .block(Block::default().borders(Borders::ALL).title("password"));
    f.render_widget(password, chunks[1]);


    // login input button, color blue if selected
    let login = match user_authenticate.selected_chunk{
        SelectedChunk::LoginButton=>{
            Paragraph::new("Login")
            .style(Style::default().fg(Color::Black).bg(Color::Blue))
            .block(Block::default().borders(Borders::ALL).title("Login"))},
        _ => {
            Paragraph::new("Login")
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Login"))
        }
    };
    f.render_widget(login, chunks[2]);

    // create account input button, color blue if selected
    let create_account = match user_authenticate.selected_chunk{
        SelectedChunk::CreateAccountButton=>{
            Paragraph::new("Create Account")
            .style(Style::default().fg(Color::Black).bg(Color::Blue))
            .block(Block::default().borders(Borders::ALL).title("Create"))
        },
        _ => {
            Paragraph::new("Create Account")
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Create"))
        },
    };
    f.render_widget(create_account, chunks[3]);

    // set the curser for username and password input
    match user_authenticate.selected_chunk{
        SelectedChunk::UsernameInput =>{
            f.set_cursor(chunks[0].x + user_authenticate.username_input.len() as u16 + 1, chunks[0].y+1);
        },
        SelectedChunk::PasswordInput =>{
            f.set_cursor(chunks[1].x + user_authenticate.password_input.len() as u16 + 1, chunks[1].y+1);
        },
        _ =>{},
    }

    // message output box
    let mut message_text = Text::from(Spans::from(user_authenticate.message.as_ref()));
    message_text.patch_style(Style::default());
    let message = Paragraph::new(message_text)
        .block(Block::default().borders(Borders::ALL).title("Message"));
    f.render_widget(message, chunks[4]);
}