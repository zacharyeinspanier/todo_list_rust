pub mod user{

    pub struct User{
        user_id: u32,
        username: String,
        password: String,
    }

    impl User{   
        pub fn user_login(user_id: u32, username: String, password: String) -> User{
            User{
                user_id,
                username,
                password,
            }
        }
        pub fn get_user_id(&self)->u32{
            return self.user_id;
        }
        pub fn get_username(&self)->String{
            return self.username.clone();
        }
        pub fn get_password(&self)->String{
            return self.password.clone();
        }

        pub fn change_username(&mut self, new_username: String){
            self.username = new_username;
        }
        pub fn change_password(&mut self, new_password: String){
            self.password = new_password;
        } 
    }
}