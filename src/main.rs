/* a terminal that recieves commands and operates on it.
basically a todolist with author's name
commands:
    - exit
    - post task
    - complete task
    - view all task with status
 */ // ⬅️ old comment


mod todo;
use todo::User;
use std::io::Write;

fn main() {
    std::process::Command::new("clear").status().unwrap(); //terminal clear

    /* capturing state */
    let state = &mut User::start();


    loop {
        print!(">> ");
        // learning command
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        // performing functions based on that command
        match input.trim() {
            "exit" => break, //✅
            "post" => User::post_list(state), //❌
            "check" => { //❌
                match User::edit_status(state) {
                    Ok(_) => println!("success!"),
                    Err(e) => println!("{}", e ),
                }
            },
            "view" => User::show_tasks(state), //❌
            "all" => User::show_everything(state), //❌
            _ => {}
        }

    }   
}