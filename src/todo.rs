use std::io::Write;

pub struct State(Vec<User>);


impl State {
    fn initialize() -> Self {
        Self(Vec::new())
    }

    fn push_it(&mut self, x: User) {
        self.0.push(x);
    }
}

#[derive(Debug)]
pub struct User {
    name: String,
    list: Vec<todo>,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct todo {
    sno: u8,
    task: String,
    status: bool,
}

impl todo {
    fn create(state: &State, owner: &str) -> Self {
        let mut todo = String::new();
        print!("Task:\n?\t");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut todo).unwrap();

        let mut count_max: u8 = 0;
        let mut newuser = true;

        for i in &state.0 {
            if i.name == owner.to_owned() {
                newuser = false;
                for j in &i.list {
                    if count_max < j.sno {
                        count_max = j.sno
                    }
                }
            } 
        }
        count_max += 1;

        
        print!("list created!:\nEntry ~ {count_max}.{todo}");
        if newuser == true { println!("\t[info: new user detected!]"); }
        println!();
        Self { sno: count_max, task: todo.trim().to_string(), status: false }

    }

    fn get_status(&self) -> bool {
        self.status
    }

    fn toggling_status(&mut self) { 
        if self.status == false {
            self.status = true;
        } else if self.status == true {
            self.status = false;
        }
    }

}

impl User {
    pub fn start() -> State { //creating a global instance
        State::initialize()
    }

    pub fn post_list(server: &mut State) {
        // get name
        let mut author = String::new();
        print!("Who's list?: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut author).unwrap();
        author = author.trim().to_string();

        // get task
        let task = todo::create(server, &author);
        
        // combine to make a list || make an instance && store it in State;
        let created_list = User { name: author, list: vec![task] };
        State::push_it(server, created_list)
    }

    pub fn show_tasks(server: &State) {
        // get user
        print!("which user?: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        
        // get list
        for i in &server.0 {
            if i.name == input {
                for j in &i.list {
                    print!("{}. {} ", j.sno, j.task);
                    if j.status == true {
                        print!("✔️");
                    }
                    println!();
                }
            }
        }
        println!();

        // let user = server.0.iter().filter(|x| x.name==input).next();
        // // match user
        // let response = server.0.iter().filter(|x| )

    }

    pub fn edit_status(server: &mut State) -> Result<(), Box<dyn std::error::Error>> {

        // get info 
            // NAME
            print!("which user?: ");
            let mut user = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut user).unwrap();
            user = user.trim().to_string();

            // TASK
            print!("target no.?: ");
            let mut sequence = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut sequence).unwrap();
            let serialnombar: u8 = sequence.trim().parse().expect("Failed to parse number");

        // access & edit
        for i in server.0.iter_mut() {
            if i.name == user {
                for j in i.list.iter_mut() {
                    if j.sno == serialnombar {
                        match j.get_status() {
                            false => {
                                println!("status is marked as undone, marking it as done...");
                                j.toggling_status();
                            },
                            true => {
                                println!("status is marked as done, marking it as undone...");
                                j.toggling_status();
                            }
                        }
                    } else {
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Serial No. INVALID 🤨")));
                    }
                }
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Username INVALID 🤨")));
            }
        }
        
        Ok(())

    }

    pub fn show_everything(server: &State) {
        println!("all users:\n{:?}", server.0);
    }
}

/* how to get a task? search thru all_users for i in self.0 where i.name == required, then get the task */