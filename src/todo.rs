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
    pass: String,
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
            get_input(Some(QUESTION::GetUser)).expect("Failed to read the input: USER"); //I'M GONNA LET YOU PANIC!
            
        match server.0.iter().any(|user| &user.name == &username) {
            true => {
                println!("Welcome Back!");
                let key: String = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                if let Some(user) = server.0.iter().find(|user| &user.name == &username && &user.pass == &key) {
                    println!("Welcome Back!");
                    return user.name.clone();
                }
                return username;
            },
            false => {
                println!("Creating New User...");
                let key = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                server.create_newuser(User { // calling server function
                    name: username.clone(),
                    pass: key,
                    todo_count: 0,
                    todo: None,
                });
                return username;        
            },
    }
    }


    pub fn task(server: &mut State, user: &String) {

        // getting task
        let task: String = get_input(Some(QUESTION::GetTask)).expect("Failed to read the input: TASK");    
        
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
        time_out(); //3s
        clear_terminal();
    }
}

/* WRAPPER FUNCTIONS */
fn save_state(state: &State) -> Result<(), Box<dyn Error>> {

    let records = to_string_pretty(&state.0)?;
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open("db.json")?;
    file.write_all(records.as_bytes())?;
    Ok(())
    
}

fn deserialize_it() -> Result<Vec<User>, Box<dyn Error>> {

    let file = std::fs::OpenOptions::new().create(true).append(true).read(true).open("db.json")?;
    let reader = BufReader::new(file);
    let data: Vec<User> = serde_json::from_reader(reader)?;

    Ok(data)
}