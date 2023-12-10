pub mod database{

    use rusqlite::{params, Connection, Result};
    use crate::todo::todo::TodoList;
    use crate::todo_item::todo_item::TodoItem;


    pub enum TodoDatabaseErrorCode{
        NoUserFound,
        ListNotFound,
        ItemNotFound
    }

    pub struct TodoDatabaseError{
        id: u32,
        message: String,
        code: TodoDatabaseErrorCode
    }

    /*
        This structure stores data from a query in the users table

        Members:
            user_id: u32 PRIMARY KEY unique identifer for a user
            username: String the username for login
            password: String the password for login
    */
    pub struct QueryUser{
        pub user_id: u32,
        pub username: String,
        pub password: String,
    }
    /*
        This structure stores data from a query in the lists table

        Members:
            user_id: u32 FORIGEN KEY the user that owns this list
            list_id: u32 PRIMARY KEY unique identifer for a list
            list_name: String the name of the list
    */
    pub struct QueryLists{
        pub user_id: u32,
        pub list_id: u32,
        pub list_name: String,
    }
    /*
        This structure stores data from a query in the items table

        Members:
            item_id: u32 PRIMARY KEY unique identifer for a item
            list_id: u32 FORIGEN KEY the list that owns this item
            tem_name: String the item name
            complete: u32 the status of complete 1 = true, 0 = false
    */
    pub struct QueryItems{
        pub item_id: u32,
        pub list_id: u32,
        pub item_name: String,
        pub complete: u32,
    }
    
    /*
        This structure is a TodoDatabase used to store
        data for a todo list

        Members:
            file_path: String path to the database file
            connection: Connection to the database
    */
    pub struct TodoDatabase{
        connection: Connection,
    }

    impl TodoDatabase{
        /*
            This Method creates a new TodoDatabase struct

            Prams: the path to the database file
            Returns: TodoDatabase struct
        */
        pub fn new(file_path: String) ->  TodoDatabase{

            // create connection
            let connection =  match Connection::open(&file_path){
                Ok(res) =>{res},
                Err(err)=>{panic!("Cound not create connection: {}", err);},
            };

            // create TodoDatabase
            let new_db = TodoDatabase{
                connection,
            };

            // create the tabels
            match new_db.build_db(){
                Ok(()) =>{},
                Err(err)=>{panic!("Cound not build database: {}", err);},
            };

            return new_db;
        }
        pub fn close_connection(self)->Result<(), rusqlite::Error>{
            self.connection.close();
            Ok(())
        }

        /*
            This Method creates all the tables for a TodoDatabase
            Prams: self

            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn build_db(&self)-> Result<(), rusqlite::Error>{

            // User table
            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS users (
                     user_id INTEGER PRIMARY KEY,
                     username TEXT,
                     password TEXT,
                     UNIQUE(username)
                 );",
                ()
            )?;

            // List table
            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS lists (
                    list_id INTEGER PRIMARY KEY,
                    user_id INTEGER,
                    list_name TEXT,
                    FOREIGN KEY(user_id) REFERENCES users(user_id) 
                 );",
                (),
            )?;

            // Items table
            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS items (
                    item_id INTEGER PRIMARY KEY,
                    list_id INTEGER,
                    item_name TEXT,  
                    complete INTEGER,
                    FOREIGN KEY(list_id) REFERENCES lists(list_id) 
                 );",
                ()
            )?;

            return Ok(());
        }

        /*
            This method creates a user in the user table.

            Prams:
                self:
                username: the username for the account
                password: the password for the account
                user_id: the unique identifier for a user

            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn create_user_account(&self, username: &str, password: &str, user_id: u32)-> Result<(), rusqlite::Error>{

            self.connection.execute(
                "INSERT INTO users (username, password, user_id) values(?, ?, ?);",
                params![username, password, user_id]
            )?;

            Ok(())
        }
        /*
            This method queries the database for a matching username and password

            Prams:
                username: account username
                password: account password
            
            Returns: Result< Ok(Vec<QueryUser>), Err>
                Ok(Vec<QueryUser>): A vector of QueryUser will all matching users
                Err: there was an error while running the SQL commands
        */
        pub fn get_user_id(&self, username: &str, password: &str) -> Result<Vec<QueryUser>, rusqlite::Error>{

            // prepares the query to execute
            let mut qury_user_id = self.connection.prepare(
                "
                SELECT *
                FROM users
                WHERE username = ?
                AND
                password = ?;
                "
            )?;

            // Execute the query and map results to QueryUser
            let rows = qury_user_id
                .query_map(
                    params![username, password], |row| Ok(QueryUser{
                        user_id: row.get("user_id")?,
                        username: row.get("username")?,
                        password: row.get("password")?,
                    }
                )
            )?;

            // Collect all results
            let collected: Result<Vec<QueryUser>, rusqlite::Error> = rows.collect();

            return collected;      
        }

        /*
            This method finds all lists and items that belong to a user_id

            Prams:
                user_id: the unique identifier for a user

            Returns: Vector of TodoLists
        */
        pub fn load_user_data(&self, user_id: u32) -> Vec<TodoList>{

            // get a vec of all list rows with user_id
            // create empty vec if user was not found
            let user_lists: Vec<QueryLists> = match self.get_user_lists(user_id){
                Ok(res) =>{res},
                Err(_err) =>{Vec::new()}, 
            };

            // create vec of TodoLists
            let mut user_data: Vec<TodoList> = Vec::new();

            // iterate through the list rows 
            // look for items with matching list_id
            user_lists.iter().for_each(|list|{
                // create the TodoList
                let mut current_list = TodoList::new(list.list_name.clone(), list.list_id);

                // query data base for all items rows with list_id
                let list_items: Vec<QueryItems> = match self.get_list_items(list.list_id){
                    Ok(res) =>{res},
                    Err(_err) =>{Vec::new()},
                };

                // Create TodoItems for every item row 
                list_items.iter().for_each(|item|{
                    let mut complete_status = false;
                    if item.complete > 0{
                        complete_status = true;
                    }
                    let current_item = TodoItem::new_from_load(
                        item.item_name.clone(), 
                        item.item_id, 
                        complete_status
                    );
                    // push to current list 
                    current_list.list.push(current_item);
                });
                // for each time create and add
                user_data.push(current_list);
            });

            return user_data;
        }
        /*
            This method queries the list table to find all lists with user_id

            Prams:
                user_id: the unique identifier for a user
            
            Returns: Result< Ok, Err>
                Ok(Vec<QueryLists>): vector of all list rows found
                Err: there was an error while running the SQL commands
        */
        fn get_user_lists(&self, user_id: u32) -> Result<Vec<QueryLists>, rusqlite::Error>{

            // prepare the query
            let mut qury_list_id = self.connection.prepare(
                "
                SELECT *
                FROM lists
                WHERE user_id = ?;
                "
            )?;

            // execuet the query, map all rows to struct QueryLists
            let rows = qury_list_id
                .query_map(
                    params![user_id], |row| Ok(QueryLists{
                        user_id: row.get("user_id")?,
                        list_id: row.get("list_id")?,
                        list_name: row.get("list_name")?,
                    }
                )
            )?;

            // collect results
            let collected: Result<Vec<QueryLists>, rusqlite::Error> = rows.collect();

            // ERROR Lists not found

            return collected;
        }
        /*
            This method queries the item table to find all items with list_id

            Prams:
                list_id: the unique identifier for a list

            Returns: Result< Ok, Err>
                Ok(Vec<QueryItems>):  vector of all item rows found
                Err: there was an error while running the SQL commands
        */
        fn get_list_items(&self, list_id: u32) -> Result<Vec<QueryItems>, rusqlite::Error>{

            // prepare the query
            let mut qury_list_id = self.connection.prepare(
                "
                SELECT *
                FROM items
                WHERE list_id = ?;
                "
            )?;

            // execuet the query, map each row to QueryItems
            let rows = qury_list_id
                .query_map(
                    params![list_id], |row| Ok(QueryItems{
                        item_id: row.get("item_id")?,
                        list_id: row.get("list_id")?,
                        item_name: row.get("item_name")?,
                        complete: row.get("complete")?,

                    }
                )
            )?;

            // collect results
            let collected: Result<Vec<QueryItems>, rusqlite::Error> = rows.collect();
            
            return collected;
        }

        /*
            This method inserts a new list into the list table

            Prams:
                list_name: the name of the list
                list_id: the unique identifier for a list
                user_id: the unique identifier for a user (user_id owns this list)

            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands, list_id already exists
        */
        pub fn insert_into_list(&self, list_name: String, list_id: u32, user_id: u32) -> Result<(), rusqlite::Error>{
            self.connection.execute(
                "INSERT INTO lists (list_id, user_id, list_name) values(?, ?, ?)",
                params![list_id, user_id, &list_name,] 
            )?;
            // ERROR could not find user
            Ok(())
        }

        /*
            This method inserts a new item into the item table

            Prams:
                item_name: the name of the item
                item_id: the unique identifier for a item
                list_id: the unique identifier for a list (list_id owns this list)
                complete: complete status
                    1 = true, 
                    0 = false
            
            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands, the item_id already exists in the item table
        */
        pub fn insert_into_items(&self, item_name: String, item_id: u32, list_id: u32, complete: u32)->  Result<(), rusqlite::Error>{

            self.connection.execute(
                "INSERT INTO items (
                    item_id,
                    list_id,
                    item_name,  
                    complete)
                    values(?,?,?,?)",
                params![item_id, list_id, &item_name, complete]
            )?;
            // Could not find list ID
            Ok(())
        }

        /*
            This method remove a user from the user table

            Prams:
                user_id: the unique identifier for a user
             
            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn remove_user(&self, user_id: u32) ->  Result<(), rusqlite::Error>{

            // Get user lists
            let lists: Vec<QueryLists> = self.get_user_lists(user_id).unwrap();
            // remove all list and items
            for list in lists{
                self.remove_list(list.list_id, user_id);
            }

            // remove the list from list table
            self.connection.execute(
                "DELETE FROM users WHERE user_id = ?;",
                params![user_id]
            )?;

            Ok(())
        }

        /*
            This method remove a list from the list table

            Prams:
                list_id: the unique identifier for a list
                user_id: the unique identifier for a user
             
            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn remove_list(&self, list_id: u32, user_id: u32) ->  Result<(), rusqlite::Error>{

            // remove items owned by list_id from the item table
            self.connection.execute(
                "DELETE FROM items WHERE list_id = ?;",
                params![list_id]
            )?;

            // remove the list from list table
            self.connection.execute(
                "DELETE FROM lists WHERE list_id = ? AND user_id = ?;",
                params![list_id, user_id]
            )?;

            // Could not find List ID, could not find user Id

            Ok(())
        }

        /*
            This method remove a item from the item table

            Prams:
                item_id: the unique identifier for a item
                list_id: the unique identifier for a list
            
            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn remove_item(&self, item_id: u32, list_id: u32)->  Result<(), rusqlite::Error>{
            self.connection.execute(
                "DELETE FROM items WHERE list_id = ? AND item_id = ?;",
                params![list_id, item_id]
            )?;
            // Could not find List_id
            Ok(())

        }
        /*
            This method update a item in the item table

            Prams:
                item_id: the unique identifier for a item
                list_id: the unique identifier for a list
                complete: complete status, 1 = true, 0 = false
            
            Returns: Result< Ok, Err>
                Ok(): the SQL commands ran without error
                Err: there was an error while running the SQL commands
        */
        pub fn update_item(&self, item_id: u32, list_id: u32, complete: u32)->  Result<(), rusqlite::Error>{

            self.connection.execute(
                "
                UPDATE items 
                SET complete = ?
                WHERE item_id = ? AND list_id = ?;
                ",
                params![complete, item_id, list_id]
            )?;

            // ERROR: Could not find Item ID
            Ok(())
        }
    }
}