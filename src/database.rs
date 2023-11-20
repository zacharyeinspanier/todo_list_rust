pub mod database{

    use rusqlite::{params, Connection, Result, Rows};
    use crate::todo::todo::TodoList;
    use crate::todo_item::todo_item::TodoItem;


    pub struct QueryUser{
        pub user_id: u32,
        pub username: String,
        pub password: String,
    }
    pub struct QueryLists{
        pub user_id: u32,
        pub list_id: u32,
        pub list_name: String,
    }
    pub struct QueryItems{
        pub item_id: u32,
        pub list_id: u32,
        pub item_name: String,
        pub date_created: String,
        pub date_complete: String,
        pub complete: u32,
    }

    pub struct TodoDatabase{
        file_path: String,
        connection: Connection,
    }

    impl TodoDatabase{
        pub fn new(file_path: String, connection: Connection) ->  TodoDatabase{
            TodoDatabase{
                file_path,
                connection,
            }
        }

        pub fn build_db(&self)-> Result<(), rusqlite::Error>{

            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS users (
                     user_id INTEGER PRIMARY KEY,
                     username TEXT,
                     password TEXT
                 )",
                ()
            )?;

            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS lists (
                    list_id INTEGER PRIMARY KEY,
                    user_id INTEGER,
                    list_name TEXT,
                    FOREIGN KEY(user_id) REFERENCES users(user_id) 
                 )",
                (),
            )?;

            self.connection.execute(
                "CREATE TABLE IF NOT EXISTS items (
                    item_id INTEGER PRIMARY KEY,
                    list_id INTEGER,
                    item_name TEXT,  
                    date_created TEXT,
                    date_complete TEXT,
                    complete INTEGER,
                    FOREIGN KEY(list_id) REFERENCES users(list_id) 
                 )",
                ()
            )?;

            return Ok(());
        }
        
        pub fn create_user_account(&self, username: &str, password: &str, user_id: u32)-> Result<(), rusqlite::Error>{

            self.connection.execute(
                "INSERT INTO users (username, password, user_id) values(?, ?, ?);",
                params![username, password, user_id]
            )?;

            Ok(())
        }
        
        pub fn get_user_id(&self, username: &str, password: &str) -> Result<Vec<QueryUser>, rusqlite::Error>{

            let mut qury_user_id = self.connection.prepare(
                "
                SELECT *
                FROM users
                WHERE username = ?
                AND
                password = ?;
                "
            )?;

            let mut rows = qury_user_id
                .query_map(
                    params![username, password], |row| Ok(QueryUser{
                        user_id: row.get("user_id")?,
                        username: row.get("username")?,
                        password: row.get("password")?,
                    }
                )
            )?;


            let collected: Result<Vec<QueryUser>, rusqlite::Error> = rows.collect();

            return collected;      
        }

        pub fn load_user_data(&self, user_id: u32) -> Vec<TodoList>{
            // Lookup the user_id in lists
            let mut user_lists: Vec<QueryLists> = match self.get_user_lists(user_id){
                Err(err) =>{Vec::new()},
                Ok(res) =>{res},
            };
            // get a vector of list_id
            let mut user_data: Vec<TodoList> = Vec::new();

            user_lists.iter().for_each(|list|{
                // create todo list
                let mut current_list = TodoList::new(list.list_name.clone(), list.list_id);
                // look up items
                let mut list_items: Vec<QueryItems> = match self.get_list_items(list.list_id){
                    Err(err) =>{Vec::new()},
                    Ok(res) =>{res},
                };

                list_items.iter().for_each(|item|{
                    let mut complete_status = false;
                    if item.complete > 0{
                        complete_status = true;
                    }
                    let current_item = TodoItem::new_from_load(
                        item.item_name.clone(), 
                        item.item_id, 
                        item.date_created.clone(), 
                        item.date_complete.clone(), 
                        complete_status
                    );
                    current_list.list.push(current_item);
                });
                // for each time create and add
                user_data.push(current_list);
            });

            return user_data;
        }

        pub fn get_user_lists(&self, user_id: u32) -> Result<Vec<QueryLists>, rusqlite::Error>{

            let mut qury_list_id = self.connection.prepare(
                "
                SELECT *
                FROM lists
                WHERE user_id = ?;
                "
            )?;


            let mut rows = qury_list_id
                .query_map(
                    params![user_id], |row| Ok(QueryLists{
                        user_id: row.get("user_id")?,
                        list_id: row.get("list_id")?,
                        list_name: row.get("list_name")?,
                    }
                )
            )?;

            let collected: Result<Vec<QueryLists>, rusqlite::Error> = rows.collect();

            return collected;


        }
        pub fn get_list_items(&self, list_id: u32) -> Result<Vec<QueryItems>, rusqlite::Error>{
            let mut qury_list_id = self.connection.prepare(
                "
                SELECT *
                FROM items
                WHERE list_id = ?;
                "
            )?;


            let mut rows = qury_list_id
                .query_map(
                    params![list_id], |row| Ok(QueryItems{
                        item_id: row.get("item_id")?,
                        list_id: row.get("list_id")?,
                        item_name: row.get("item_name")?,
                        date_created: row.get("date_created")?,
                        date_complete: row.get("date_complete")?,
                        complete: row.get("complete")?,

                    }
                )
            )?;

            let collected: Result<Vec<QueryItems>, rusqlite::Error> = rows.collect();
            
            return collected;
        }

        
        pub fn insert_into_list(&self, list_name: String, list_id: u32, user_id: u32) -> Result<(), rusqlite::Error>{
            self.connection.execute(
                "INSERT INTO lists (list_id, user_id, list_name) values(?, ?, ?)",
                params![list_id, user_id, &list_name,] 
            )?;
            Ok(())
        }
        
        pub fn insert_into_items(&self, item_name: String, item_id: u32, list_id: u32, date_created: String, date_complete: String, complete: u32)->  Result<(), rusqlite::Error>{

            self.connection.execute(
                "INSERT INTO items (
                    item_id,
                    list_id,
                    item_name,  
                    date_created,
                    date_complete,
                    complete)
                    values(?,?,?,?,?,?)",
                params![item_id, list_id, &item_name, &date_created, &date_complete, complete]
            )?;
            Ok(())
        }
        
        pub fn remove_list(&self, list_id: u32, user_id: u32) ->  Result<(), rusqlite::Error>{
            // first remove all items
            self.connection.execute(
                "DELETE FROM items WHERE list_id = ?;",
                params![list_id]
            )?;

            self.connection.execute(
                "DELETE FROM lists WHERE list_id = ? AND user_id = ?;",
                params![list_id, user_id]
            )?;

            Ok(())
        }
        
        pub fn remove_item(&self, item_id: u32, list_id: u32)->  Result<(), rusqlite::Error>{
            self.connection.execute(
                "DELETE FROM items WHERE list_id = ? AND item_id = ?;",
                params![list_id, item_id]
            )?;

            Ok(())

        }
        pub fn update_item(&self, item_id: u32, list_id: u32, complete: u32, date: String)->  Result<(), rusqlite::Error>{

            self.connection.execute(
                "
                UPDATE items 
                SET complete = ?, date_complete = ?
                WHERE item_id = ? AND list_id = ?;
                ",
                params![complete, date, item_id, list_id]
            )?;

            Ok(())
        }
    }
}