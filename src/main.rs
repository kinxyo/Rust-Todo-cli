mod todo;
use crate::todo::User;

mod functions;
use crate::functions::{clear_terminal, get_input}; //need to unimport it later

/* create a error.txt log and update errors to it whenever they occur. Also once you get that working, add a functionality where the error files are uploaded to you on some server, so we can read those errors and fix the codebase. */

fn main() {

    /* WELCOME STAGE */

        // clear_terminal(); //don't comment this out until production.
        
        //creates an instance & loads existing data (if any)
        let state = &mut User::welcome();

        // get user handle (be it existing or new)
        let user = User::handle(state);
        
        // refresh the terminal and post instructions
        clear_terminal(); println!("( Add Task: \"post\" | Show Records: \"all\" | exit )");
    

    /* MAIN STAGE */
        loop {        

            let command: String = get_input(None).expect("Failed to read the input: COMMAND");

            match &command[..] {
                "exit" => {
                    User::close(&state);
                    break;
                },
                "post" => User::task(state, &user),
                "all" => User::all(state),
                // add more cases of username signout and login etc.
                _ => {}
            }
        }   
}