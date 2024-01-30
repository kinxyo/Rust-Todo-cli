mod todo;
use crate::todo::User;

mod functions;
use crate::functions::{clear_terminal, get_input};

/* create a error.txt log and update errors to it whenever they occur. Also once you get that working, add a functionality where the error files are uploaded to you on some server, so we can read those errors and fix the codebase. */

fn main() {
    
    // clear_terminal(); //don't comment this out until production.
    

    let state = &mut User::start();

    state.load_state();

    let username = User::handle(state);
    
    // refresh
    clear_terminal();
    println!("( Add Task: \"post\" | Show Records: \"all\" | exit )");
    
    loop {        

        let command: String = get_input(None).expect("Failed to read the input: COMMAND"); //TEST: error handling (although I don't think a `match` handle would be needed here).

        match command.as_str() {
            "exit" => {
                clear_terminal();
                break
            },
            "post" => User::task(state, &username),
            "all" => User::all(state),
            // add more cases of username signout and login etc.
            _ => {}
        }
    }   
}