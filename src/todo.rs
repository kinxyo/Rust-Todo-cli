/* IMPORTS  */
use serde::{Deserialize, Serialize};
use crate::functions::{log_error, clear_terminal, get_input, time_out, QUESTION};
use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::error::Error;
use serde_json::to_string_pretty;
use std::option::Option::Some;
use dialoguer::Select;


/* STRUCTS */
#[derive(Debug)]
pub struct State(HashMap<String, User>); //Using `String`(username) as key because this is a small application, and I often need to access the user data by username. If this was a large application, I would have used `u64` (user_id) as key. This is because `u64` is faster to access than `String` (because of hashing).

    /* 
        DO NOTE: 
            
            1. `state.0` | `self.0` | `server.0` refers to the tuple of tuple struct.

            2.  so essentially,
                 `state.0` = `Vec<User>` (is refering to the vector of user associated data).
    
    */ 

#[derive(Serialize, Deserialize, Debug, Clone)] //clone will occur explicitly only when registering a new user (hence not expensive)
pub struct User {
    pass: String,
    todo_count: u8,
    todo: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct Task {
    title: String,
    status: bool,
}

/* `IMPL` */
impl State {

    fn initialize() -> Self {
            Self(HashMap::new()) //creates a new instance
    }

    fn post_task(&mut self, user: &String, task: Task) {
        
        let userdata = self.0.get_mut(user).unwrap();

        match &mut userdata.todo {
            Some(list) => list.push(task),
            None => userdata.todo = Some(vec![task]),
        }

        userdata.todo_count += 1;
    }

    fn load_state(&mut self) { 

        //first we open the 'saved data' file to check if it's empty (newly created)?
            //if yes
                //we do nothing
            //else
                //we load the state


        let file = std::fs::OpenOptions::new().create(true).append(true).read(true).open("db.json").expect("Failed to access the `state`"); //I want the program to crash here because if `db.json` is not created (when not present) then there will be no 'state preservation', the program will be of no use. However, this is very unlikely to happen & probably because of permission issues. I'll cover how to resolve this in documentation. I cannot cover it here because there is nothing I can do to get permission, this solely lies on user's end.
        
        match check_file("db.json") {
            Ok(false) => {},
            Ok(true) => {
                println!("Loading the state...");
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(data) => {
                        println!("Loaded the `state`: {:#?}", &data);
                        self.0 = data;
                        println!("state now: {:#?}", &self.0); 
                    },
                    Err(e) => {
                        log_error(&e, Some("Saved Data Corrupted!"));
                    },
                }
            },
            Err(e) => log_error(&*e, Some("Failed to Check the `state`")),
        }

    } 
    

}


impl User {

    pub fn welcome() -> State {

        //checks if an existing `state` exists
            // if yes
                //loads the state
            // else 
                //returns an instance
        
        let mut state = State::initialize();
        state.load_state();
        println!("state captured: {:#?}", &state);
        return state;
        
    }

    pub fn handle(server: &mut State) -> String { // returns `Username`

        let mut choices: Vec<String> = Vec::new();

        // show all users
        for (key, _) in &server.0 { //will come back to this later when the program is working
            choices.push(key.to_owned());
        }

        choices.push(String::from("Create a new User"));


        // shows all logged in users ✅
            //choose between them using arrow keys
                //chose: existing account
                    // password validation
                //chose: create a new account
                    // creates a new account
                    // restarts the program
            
            
            
            let index = Select::new()
            .with_prompt("Choose a user")
            .items(&choices)
            .default(0)
            .interact()
            .expect("terminal doesn't support interactive mode");

            let select = choices[index].to_owned();

            
            if  select == "Create a new User" {
                println!("Creating New User...");
                
                let username: String = get_input(Some(QUESTION::GetUser)).expect("Failed to read the input: USER"); //I'M GONNA LET YOU PANIC!
                let key: String = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                
                let newuser = User { pass: key, todo_count: 0, todo: None, };
                
                let _ = &server.0.insert(username.clone(), newuser);
                
                return username;
                
            } else {
                // login
                let key: String = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                    
                    let user = &server.0.get(&select).expect("Failed to get the user");

                    if user.pass == key {
                        return select;
                    } else {
                        panic!("Wrong Password!");
                    }
        }



    }
    

    pub fn task(server: &mut State, useracc: &String) {

        // getting task
        let task: String = get_input(Some(QUESTION::GetTask)).expect("Failed to read the input: TASK");    
        
        // calling server function
        server.post_task(useracc, Task { title: task, status: false });

    }
 
    pub fn all(server: &State) {
        println!("all users:\n{:#?}", server.0);
        time_out(); // 3s
        clear_terminal();
    }

    pub fn close(server: &State) {
        clear_terminal();
        // saving the state
        match save_state(&server) {
            Ok(_) => {
                println!("Saved the `state`: {:#?}", &server);
                time_out();
            },
            Err(e) => log_error(&*e, Some("Failed to Save the `state`")),
        }
    }

}

/* WRAPPER FUNCTIONS */
fn save_state(state: &State) -> Result<(), Box<dyn Error>> {

    let records = to_string_pretty(&state.0)?;
    let mut file = std::fs::OpenOptions::new().create(true).write(true).append(false).open("db.json")?;
    file.write_all(records.as_bytes())?;
    Ok(())
    
}

fn check_file(file: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if std::fs::metadata(file)?.len() == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}