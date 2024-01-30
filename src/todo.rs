/* IMPORTS  */
use serde::{Deserialize, Serialize};
use crate::functions::{log_error, clear_terminal, get_input, time_out, QUESTION};
use std::cell::RefCell;
use std::io::{BufReader, Write};
use std::error::Error;
use serde_json::to_string_pretty;
use std::option::Option::Some;
use dialoguer::Select;


/* STRUCTS */
pub struct State(Vec<User>); //RefCell will be used here to

    /* 
        DO NOTE: 
            
            1. `state.0` | `self.0` | `server.0` refers to the tuple of tuple struct.

            2.  so essentially,
                 `state.0` = `Vec<User>` (is refering to the vector of user associated data).
    
    */ 

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

    fn initialize() -> Self {
            Self(Vec::new()) //creates a new instance
    }

    fn register_newuser(&mut self, user: &User) -> &User {
            self.0.push(*user);
            &user
    }

    fn post_task(&mut self, userdata: RefCell<User>, task: Task) {

            let mut user = userdata.borrow_mut();

            match &mut user.todo {
                Some(mut todo) => todo.push(task),
                None => user.todo = Some(vec![task]),
            }

            user.todo_count += 1;
    }

    fn load_state(&mut self) { 

        //first we open the 'saved data' file to check if it's empty (newly created)?
            //if yes
                //we do nothing
            //else
                //we load the state


        let file = std::fs::OpenOptions::new().create(true).append(true).read(true).open("db.json").expect("Failed to access the `state`"); //I want the program to crash here because if `db.json` is not created (when not present) then there will be no 'state preservation', the program will be of no use. However, this is very unlikely to happen & probably because of permission issues. I'll cover how to resolve this in documentation. I cannot cover it here because there is nothing I can do to get permission, this solely lies on user's end.
        
        match check_file("db.json") {
            Ok(true) => {},
            Ok(false) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(data) => self.0 = data,
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
        return state;
        
    }

    pub fn handle(server: &mut State) -> &User { // returns `Username`

        // shows all logged in users
            //choose between them using arrow keys
                //chose: existing account
                    // password validation
                //chose: create a new account
                    //creates a new account
                    // restarts the program

            
            let mut all_users = server.0.iter().map(|user| &user.name).collect::<Vec<&String>>();
            
            let new_option = String::from("Create a new User");
            all_users.push(&new_option);

            

            let selected_index = Select::new()
            .with_prompt("Choose a user")
            .items(&all_users)
            .default(0)
            .interact()
            .unwrap();

        let selected_option: &String = all_users[selected_index];
        let selected_option: String = selected_option.to_owned();

        if  selected_option == "String" {
                let username: String = get_input(Some(QUESTION::GetUser)).expect("Failed to read the input: USER"); //I'M GONNA LET YOU PANIC!

                println!("Creating New User...");
                let key = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                server.register_newuser(&User { // register user and returns it as well
                    name: username.clone(),
                    pass: key,
                    todo_count: 0,
                    todo: None,
                })      
        } else {
                //login
                let key: String = get_input(Some(QUESTION::GetPassword)).expect("Failed to read the input: PASS");
                let user = server.0.iter().find(|user| &user.name == &selected_option && &user.pass == &key).unwrap(); //there is no chance of error.
                println!("Welcome Back!");
                return &user;
        }
    }
    

    pub fn task(server: &mut State, userdata: &User) {

        let mut user  = RefCell::new(*userdata);


        // getting task
        let task: String = get_input(Some(QUESTION::GetTask)).expect("Failed to read the input: TASK");    
        
        // calling server function
        server.post_task(user, Task { title: task, status: false });

    }
 
    pub fn all(server: &State) {
        println!("all users:\n{:#?}", server.0);
        time_out(); //3s
        clear_terminal();
    }

    pub fn close(server: &State) {
        clear_terminal();
        // saving the state
        match save_state(&server) {
            Ok(_) => {},
            Err(e) => log_error(&*e, Some("Failed to Save the `state`")),
        }
    }

}

/* WRAPPER FUNCTIONS */
fn save_state(state: &State) -> Result<(), Box<dyn Error>> {

    let records = to_string_pretty(&state.0)?;
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open("db.json")?;
    file.write_all(records.as_bytes())?;
    Ok(())
    
}

fn check_file(file: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if std::fs::metadata(file)?.len() == 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}