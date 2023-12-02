pub mod user{
    /*
        The structure for User

        Members:
            user_id: u32 uniqly identifys a user
            username: String username for login
            password: String password for login
    */
    pub struct User{
        user_id: u32,
        username: String,
        password: String,
    }

    impl User{  
        /*
            Creates a User structure

            Prams:
                user_id: u32 uniqly identifys a user
                username: String username for login
                password: String password for login
            Returns: A user structure
        */ 
        pub fn new(user_id: u32, username: String, password: String) -> User{
            User{
                user_id,
                username,
                password,
            }
        }

        /*
            Returns: u32 the user id
        */
        pub fn get_user_id(&self)->u32{
            return self.user_id;
        }

        /*
            Returns: A String clone of the username
        */
        pub fn get_username(&self)->String{
            return self.username.clone();
        }

        /*
            Returns: A String clone of the password
        */
        pub fn get_password(&self)->String{
            return self.password.clone();
        }

        /*
            Sets a new username

            Pram: 
                new_username: String the new username for User
        */
        pub fn change_username(&mut self, new_username: String){
            self.username = new_username;
        }

         /*
            Sets a new password
            
            Pram: 
                new_password: String the new password for User
        */
        pub fn change_password(&mut self, new_password: String){
            self.password = new_password;
        } 
    }
}