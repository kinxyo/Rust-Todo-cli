use std::io::Write;

pub fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn time_out() {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

pub enum QUESTION {
    GetUser,
    GetTask,
    GetPassword,
}


pub fn get_input<T: std::str::FromStr>(askf: Option<QUESTION>) -> Result<T, Box<dyn std::error::Error>> { // askf = "asking for?"

    match askf {
        Some(QUESTION::GetUser) => {
            
            println!("USERNAME:");
            return take_input();
        
        },
        Some(QUESTION::GetTask) => {
            
            println!("\n[ENTER TASK]:");
            let t = take_input();
            clear_terminal();
            
            println!("Task Added!");
            return t;
        },
        Some(QUESTION::GetPassword) => {
            
            println!("PASSWORD:");
            return take_input();
        
        },
        None => {
            println!("\n[COMMAND]:");
            return take_input();
        },
    }
    
}


fn take_input<T: std::str::FromStr>() -> Result<T, Box<dyn std::error::Error>> {
    loop {
        print!("\n\t?> ");
        std::io::stdout().flush()?; //`?` means it instantly returns an error instead of explicitly handling. Only works inside a function that is already returning a result.
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().parse::<T>() {
            Ok(t) => return Ok(t),
            Err(_) => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

pub fn log_error(caught_error: &dyn std::error::Error, context: Option<&str>) {
    let c: &str;
    match context {
        Some(context) => c = context,
        None => c = "Error",
    }
    let error_message = format!("{}: {:?}", c, caught_error);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("error.txt")
            .expect("Failed to open the `error log`");
        writeln!(file, "{}", error_message).expect("Error Log Failed: You were supposed to stop them not join them!");
}