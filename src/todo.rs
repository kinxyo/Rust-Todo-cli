/* IMPORTS  */
use crate::functions::{clear_terminal, get_input, log_error, time_out, check_file, QUESTION};
use dialoguer::Select;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, Write};
use std::option::Option::Some;

/*
    DO NOTE:

        1. `state.0` | `self.0` | `server.0` refers to the tuple of tuple struct.

        2.  so essentially,
             `state.0` = `Vec<User>` (is refering to the vector of user associated data).

*/

/* STRUCTS */
#[derive(Debug)]
pub struct State(HashMap<String, User>);

#[derive(Serialize, Deserialize, Debug, Clone)] //clone will occur explicitly only when registering a new user; therefore it's not expensive.
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

    fn save_state(&self) -> Result<(), Box<dyn Error>> {
        let records = to_string_pretty(&self.0)?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open("db.json")?;
        file.write_all(records.as_bytes())?;
        Ok(())
    }

    fn load_state(&mut self) {
        //first we open the 'saved data' file to check if it has some content?
        //if yes
        //we load the state
        //else
        //we do nothing

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open("db.json")
            .expect("Failed to access the `state`");

        match check_file("db.json") {
            Ok(false) => {}
            Ok(true) => {
                let reader = BufReader::new(file);

                match serde_json::from_reader(reader) {
                    Ok(data) => self.0 = data,
                    Err(e) => {
                        log_error(&e, Some("Saved Data Corrupted!"));
                    }
                }
            }
            Err(e) => log_error(&*e, Some("Failed to Check the `state`")),
        }
    }
}

impl User {
    pub fn welcome() -> State {
        let mut state = State::initialize();

        state.load_state(); //if any

        return state;
    }

    pub fn handle(server: &mut State) -> String {
        // returns `Username`

        // shows all logged in users ✅
        //choose between them using arrow keys
        //chose: existing account
        // password validation
        //chose: create a new account
        // creates a new account
        // restarts the program

        let mut choices: Vec<String> = Vec::new();
        for (key, _) in &server.0 {
            choices.push(key.to_owned());
        }
        choices.push(String::from("Create a new User"));

        let index = Select::new()
            .with_prompt("Choose a user")
            .items(&choices)
            .default(0)
            .interact()
            .expect("terminal doesn't support interactive mode");

        let select = choices[index].to_owned();

        if select == "Create a new User" {
            println!("Creating New User...");

            /* abstraction needed */

            let username: String =
                get_input(Some(QUESTION::GetUser)).expect("Failed to read the input: USER"); //I'M GONNA LET YOU PANIC!
            let key: String =
                get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");

            let newuser = User {
                pass: key,
                todo_count: 0,
                todo: None,
            };

            let _ = &server.0.insert(username.clone(), newuser);

            return username;
        } else {
            // login
            let key: String =
                get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");

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
        let task: String =
            get_input(Some(QUESTION::GetTask)).expect("Failed to read the input: TASK");

        // add task to todolist
        server.post_task(
            useracc,
            Task {
                title: task,
                status: false,
            },
        );
    }

    pub fn view(server: &State, user: &String) { 
        println!("{:#?}", &server.0.get(user).unwrap().todo);
        time_out(); // 3s
        clear_terminal();
    }

    pub fn close(server: &State) {
        
        clear_terminal();
        // saving the state
        match server.save_state() {
            Ok(_) => {
                // println!("Saved the `state`: {:#?}", &server); //for debugging purposes
                time_out();
            }
            Err(e) => log_error(&*e, Some("Failed to Save the `state`")),
        }
    }
}