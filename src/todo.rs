/* IMPORTS  */
use serde::{Deserialize, Serialize};
use crate::functions::{log_error, clear_terminal, get_input, time_out, QUESTION};
use std::io::{BufReader, Write};
use std::error::Error;
use serde_json::to_string_pretty;
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
            } else { // Handle the case where user.todo is None, e.g. by creating a new todo list
                user.todo = Some(vec![task]); 
            }

            user.todo_count += 1;
        }

    }

    pub fn load_state(&mut self) { 
        match deserialize_it() {
            Ok(data) => {
                self.0 = data;
            },
            Err(e) => {
                if let Some(json_err) = e.downcast_ref::<serde_json::Error>() {
                    if let serde_json::error::Category::Eof = json_err.classify() {
                        // Handle EOF error here
                        println!("No Previous State Found!");
                    } else {
                        log_error(&*e, Some("Failed to Load the `state`"));
                    }
                }
            },
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
        match save_state(&server) {
            Ok(_) => {},
            Err(e) => log_error(&*e, Some("Failed to Save the `state`")),
        }

    }
 
    pub fn all(server: &State) {
       
        println!("all users:\n{:#?}", server.0);
        
        // Timeout: 3sec
        time_out();
        clear_terminal();
    }
}

/* State Functions */
fn save_state(state: &State) -> Result<(), Box<dyn Error>> {

    let records = to_string_pretty(&state.0)?;
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open("db.json")?;
    file.write_all(records.as_bytes())?;
    Ok(())
    
}

// Post This In Readme.md ⬇️

/*
    LETTING SOME ERRORS PANIC BECAUSE HANDLING THEM IS OVERKILL.
     For past 2 hours, I've been obsessively searching all the functions 
     that return a `result` so I can use `match` on them. 
     However, I feel as though that there are some errors that are never going to happen under normal circumstances,
     and even if they do, I don't really know how I will keep the programing running. 
     I want the program to crash in such extremely rare cases.

    (asking copilot because I have nobody else to ask to)
        > Yes, in this specific case where your application is a CLI todo-list app and the input is always a string,
        it might be acceptable to let the program panic if it fails to read the input.
        As you've mentioned, there's not much you can do to recover from this error, 
        and it's unlikely to occur under normal circumstances.
*/

/* Wrapper functions: 
    In code there are a lot of methods return a `Result`. 
    I can not go and individually apply `match` case to them (achulli, I did just that but it made code very ugly)
    Hence, I use `?` operator on them,
    and then I wrap them in a function that returns `Result` and then I apply `match` case on that function.
    This way, I can keep the code clean and also handle the errors.
*/

fn deserialize_it() -> Result<Vec<User>, Box<dyn Error>> {

    let file = std::fs::OpenOptions::new().create(true).append(true).read(true).open("db.json")?;
    let reader = BufReader::new(file);
    let data: Vec<User> = serde_json::from_reader(reader)?;

    Ok(data)
}