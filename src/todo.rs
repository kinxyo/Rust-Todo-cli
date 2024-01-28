/* IMPORTS  */
use serde::{Deserialize, Serialize};
use crate::functions::{log_error, clear_terminal, get_input, time_out, QUESTION};
use std::io::{Write, Read};
use serde_json::{to_string_pretty, from_str};
use std::option::Option::Some;


/* STRUCTS */
pub struct State(Vec<User>);

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    todo_count: u8,
    todo: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct Task {
    title: String,
    status: bool,
}

/* `IMPL` */
impl State {

    /* 
        DO NOTE: 
            
            1. `server.0` | `self.0` | `state.0` refers to the tuple of tuple struct.

            2.  so essentially,
                 `self.0` = `Vec<User>` (is refering to the vector of user associated data).
    
    */ 

    fn initialize() -> Self {
        Self(Vec::new())  //Instance created!
    }

    fn create_newuser(&mut self, user: User) {


        
        self.0.push(user)
    }

    fn post_task(&mut self, username: &str, task: Task) {

        if let Some(user) = self.0.iter_mut().find(|user| user.name == username) {

            if let Some(todo) = &mut user.todo {

                todo.push(task);
            
            } else {
                
                // Handle the case where user.todo is None, e.g. by creating a new todo list
                user.todo = Some(vec![task]);
            }

            user.todo_count += 1;
        }

    }

    pub fn load_state(&mut self) { 

        // let status: bool;

        // fetching `state` from local file.
        let file = std::fs::File::open("db.json");
        match file {
            Ok(mut file) => {
                
                // loading the state        
                let mut data = String::new();
                
                match file.read_to_string(&mut data) {
                    Ok(_) => (),
                    Err(e) => log_error(&e, Some("Failed to read from db.json")),
                }
                
                match from_str(&data) {
                    Ok(val) => {
                        self.0 = val;
                    },
                    Err(e) => log_error(&e, Some("Failed to deserialize the data")),
                }
            },
            Err(e) => log_error(&e, Some("Failed to open the file")),
        }
    
    }
    

}

/* `State` performs all the actions as it's its instance that's holding everything */
/* `User` is merely just an abraction layer */


impl User {
    pub fn start() -> State {
        State::initialize() // Creating a `Global` instance of `State`.
        // let server = &mut State::initialize(); // Creating a `Global` instance of `State`.
        // server.load_state();
    }

    pub fn handle(server: &mut State) -> String { // returns `Username`
                
        let username: String =
            get_input(Some(QUESTION::FindUser)).expect("Failed to read the input: USER"); //I'M GONNA LET YOU PANIC!

        /* 2 choices -- username_yes, username_no */


        // username_yes
        if server.0.iter().any(|user| &user.name == &username) {
            return username;
        }

        // username_no
        server.create_newuser(User { // calling server function
            name: username.clone(),
            todo_count: 0,
            todo: None,
        });
        return username;

    }

    pub fn task(server: &mut State, user: &String) {

        // getting task
        let task: String = get_input(Some(QUESTION::FindTask)).expect("Failed to read the input: TASK");    
        
        // calling server function
        server.post_task(user, Task { title: task, status: false });
        
        // saving the state
        save_state(&server);

    }
 
    pub fn all(server: &State) {
       
        println!("all users:\n{:#?}", server.0);
        
        // Timeout: 3sec
        time_out();
        clear_terminal();
    }
}


fn save_state(state: &State) {
    
    // serializing the data
    let mut records: String = String::new();
    match to_string_pretty(&state.0) {
        Ok(val) => {
            records = val;
        },
        Err(e) => {
            log_error(&e, Some("Failed to serialize the data"));
        }
    };
    
    // writing the data to the file
    match std::fs::OpenOptions::new().create(true).write(true).open("db.json") {
        Ok(mut file) => {
            match file.write_all(records.as_bytes()) {
                Ok(_) => {},
                Err(e) => {
                    log_error(&e, Some("Failed to write to `db.json`"));
                }
            }
        },
        Err(e) => {
            log_error(&e, Some("Failed to open `db.json`"));
        }
    }
}