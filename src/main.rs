mod todo;
use crate::todo::User;

mod functions;
use crate::functions::{clear_terminal, get_input};

/* create a error.txt log and update errors to it whenever they occur. Also once you get that working, add a functionality where the error files are uploaded to you on some server, so we can read those errors and fix the codebase. */

fn main() {
    
    clear_terminal(); //don't comment this out until production.
    

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