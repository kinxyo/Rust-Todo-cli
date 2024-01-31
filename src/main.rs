mod todo;
use crate::todo::User;

mod functions;
#[allow(unused_imports)]
use crate::functions::{clear_terminal, get_input}; //need to unimport it later

fn main() {

    /* WELCOME STAGE */

        // clear_terminal(); //don't comment this out until production.
        
        //creates an instance & loads existing data (if any)
        let state = &mut User::welcome();

        // get user handle (be it existing or new)
        let user: String = User::handle(state);
        
        // refresh the terminal and post instructions
        // clear_terminal(); println!("( Add Task: \"post\" | Show Records: \"all\" | exit )");
    

    /* MAIN STAGE */
        loop {        

            let command: String = get_input(None).expect("Failed to read the input: COMMAND");

            match &command[..] {
                "exit" => {
                    User::close(&state);
                    break;
                },
                "post" => User::task(state, &user),
                "view" => User::view(state, &user), //get a glance of the user's todo list
                // add more cases of username signout and login etc.
                _ => {}
            }
        }   
}