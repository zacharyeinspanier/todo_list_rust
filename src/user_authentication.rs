pub mod user_authentication{

    use crate::database::database::{TodoDatabase, QueryUser};
    use crate::user::user::User;
    use rand;

    /*
        This enum sets the state for Authentication struct

        members:
            Default: User can quit app, or enter UserInput state
            UserInput: User can enter input for login or create account
            LoggedIn: User is logged in and the app exits
    */
    pub enum AuthenticationState{
        Default,
        UserInput,
        LoggedIn,
    }

    /*
        This enum sets the current input box the user selects

        members:
            UsernameInput: Allow user to enter keys for username
            PasswordInput:  Allow user to enter keys for password
            LoginButton: Allow user to press enter to login
            CreateAccountButton: Allow user to press enter to create an account
    */
    pub enum SelectedChunk{
        UsernameInput,
        PasswordInput,
        LoginButton,
        CreateAccountButton,
    }

    /*
        This struct is an Authentication used to store data for authenitcating a user

        Members:
            authentication_state: AuthenticationState state of the app
            username_input: String, username input box
            password_input: String, password input box
            index: u32, the index use to select SelectedChunk
            database: TodoDatabase, the database for login and create account
            selected_chunk: SelectedChunk, the currently selected input box
            message: String, the message to display for feedback to user
            max_index: u32, the max that index can less than
            user_id: u32, the user_id from login
            username: String, the username from login
            password: String, the password from login
    */
    pub struct Authentication <'a>{
        pub authentication_state: AuthenticationState,
        pub username_input: String,
        pub password_input: String,
        pub index: u32,
        pub database: &'a TodoDatabase, 
        pub selected_chunk: SelectedChunk, 
        pub message: String,
        max_index: u32,
        user_id: u32,
        username: String,
        password: String,
        
    }

    impl Authentication <'_>{

        /*
            This method creats a new Authentication struct

            Prams:
                database: TodoDatabase, the database for login and create account

            Returns: Authentication struct  
        */
        pub fn new(database: &TodoDatabase)->Authentication{
            Authentication{
                authentication_state: AuthenticationState::Default,
                username_input: String::new(),
                password_input: String::new(),
                message: String::from("Create an account or login!"),
                index: 0,
                database,
                selected_chunk: SelectedChunk::UsernameInput,
                max_index: 4,
                user_id: u32::MAX,
                username: String::new(),
                password: String::new(),
            }
        }

        /*
            This method takes the username and password a user enters. Then tries to log the user
        */
        pub fn login(&mut self){

            // drain username
            let username: String = self.username_input.drain(..).collect();
            let password: String = self.password_input.drain(..).collect();

            // empty strings are not allowed
            if username == "" || password == ""{
                return;
            }
            
            // query database for the user
            let user_query: Vec<QueryUser> = match self.database.get_user_id(&username, &password){
                Ok(res) => {res},
                Err(err) =>{
                    panic!("{}", err)
                },
            };

            // only one user should be found
            if user_query.len() == 1{
                // store information from query
                self.user_id = user_query[0].user_id;
                self.username = user_query[0].username.clone();
                self.password = user_query[0].password.clone();
                self.authentication_state = AuthenticationState::LoggedIn;
                self.message = String::from("User: ") + &username.clone() + &String::from(" Logged in! Press any key to proceed."); // Login message
            }
            else{
                self.message = String::from("User: ") + &username.clone() + &String::from(" does not exist. Create an account!"); // user not found message
            }       
        }

        /* 
            This method takes the username and password a user enters. Then tries to create an account
        */
        pub fn create_account(&mut self){

            // drain username and password
            let username:String = self.username_input.drain(..).collect();
            let password:String = self.password_input.drain(..).collect();

            // empty strings are not allowed
            if username == "" || password == ""{
                return;
            }

            // check if a user already exists with the same username and password
            let user_query: Vec<QueryUser> = match self.database.get_user_id(&username, &password){
                Ok(res) => {
                    res},
                Err(err) =>{panic!("{}", err)}, 
            };

            // zero users should be found
            if user_query.len() == 0{

                let mut user_id: u32 ;

                loop {
                    // generate a random user_id
                    user_id = rand::random::<u32>();
                    if user_id == u32::MAX {continue;}
                    // try to create an account, err when the user_id already exists
                    match self.database.create_user_account(&username, &password, user_id){
                        Ok(()) =>{break;},
                        Err(err) => {
                            match err.sqlite_error_code().unwrap(){
                                rusqlite::ErrorCode::ConstraintViolation => { continue;}
                                _ =>{panic!("{}", err)}
                            }
                        },
                    }
                }
                self.message = String::from("Account create for User: ") + &username.clone(); // success message
            }
            else{
                // user already exis message
                self.message = String::from("User: ") + &username.clone() + &String::from(" alredy exists, pick a new username and password");
            }
        }

        /*
            Update the index to the input box below the current box
        */
        pub fn next_index(&mut self){
            if self.index < self.max_index - 1{
                self.index += 1;
            }
            else{
                self.index = 0;
            }
            // update selected box
            self.selected_box();
        }

        /*
            Update the index to the input box above the current box
        */
        pub fn previous_index(&mut self){
            if self.index > 0{
                self.index -= 1;
            }
            else{
                self.index = self.max_index - 1;
            }
            // update selected box
            self.selected_box();
        }

        /*
            This method handels user input of enter
        */
        pub fn process_enter(&mut self){
            // if the SelectedChunk::LoginButton then login()
            // if the SelectedChunk::CreateAccountButton then create_account()
            match self.selected_chunk{
                SelectedChunk::LoginButton =>{
                    self.login();
                },
                SelectedChunk::CreateAccountButton =>{
                    self.create_account();
                },
                _=> {},
            }
        }

        /*
            This method pushes a char to username_input or password_input

            prams:
                c: char the character the user entered
        */
        pub fn add_input(&mut self, c: char){
            // if SelectedChunk::UsernameInput then push to username_input
            // if SelectedChunk::PasswordInput then push to password_input
            match self.selected_chunk{
                SelectedChunk::UsernameInput =>{
                    self.username_input.push(c);
                },
                SelectedChunk::PasswordInput =>{
                    self.password_input.push(c);
                },
                _=> {},
            }
        }

        /*
            This method handels user input of delete/backspace
        */
        pub fn remove_input(&mut self){
            // if SelectedChunk::UsernameInput then pop from username_input
            // if SelectedChunk::PasswordInput then pop from password_input
            match self.selected_chunk{
                SelectedChunk::UsernameInput =>{
                    self.username_input.pop();
                },
                SelectedChunk::PasswordInput =>{
                    self.password_input.pop();
                },
                _=> {},
            }
        }

        /*
            This method matches the current index to the SelectedChunk 
        */
        fn selected_box(&mut self){
            match self.index {
                0 =>{self.selected_chunk = SelectedChunk::UsernameInput;},
                1 =>{self.selected_chunk = SelectedChunk::PasswordInput;},
                2 =>{self.selected_chunk = SelectedChunk::LoginButton;},
                3 =>{self.selected_chunk = SelectedChunk::CreateAccountButton;},
                _ => {},
            };
        }

        /*
            This method enters changes the authentication_state to AuthenticationState::UserInput
        */
        pub fn user_input(&mut self){
            match self.authentication_state {
                AuthenticationState::Default => {
                    self.authentication_state = AuthenticationState::UserInput;
                },
                _ => {},
            };
        }

        /*
            This method enters changes the authentication_state to AuthenticationState::Default
        */
        pub fn default(&mut self){
            match self.authentication_state {
                AuthenticationState::UserInput => {
                    self.authentication_state = AuthenticationState::Default;
                },
                _ => {},
            };
        }

        /*
            This method creates a user and returns a option

            Returns:Option<User>
                Some(user): user has logged in
                None: user has not logged in

        */
        pub fn get_user(&self)->Option<User>{
            if self.user_id == u32::MAX{
                None
            }
            else{
                Some(User::new(self.user_id, self.username.clone(), self.password.clone()))
            } 
        }
    }
}