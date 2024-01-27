use std::io::Write;

/* `State` is supposed to incorporate the whole of data. 
    There isn't much data except for todolist and the user associated with that. 
    That's why `User` struct is inside the `State`.
    There isn't just 1 user but multiple hence it's wrapped in a vector.

    This can be improved by utilizing database perhaps (?)
*/

#[derive(Debug)]
pub struct State(Vec<User>); 


impl State {
    fn initialize() -> Self {
        //creates its own instance
        Self(Vec::new())
    }

    fn push_it(&mut self, x: User) {
        // receives a list of task instead of just task
        self.0.push(x); //pushes it onto itself instead of associated list.

        //you see where things are going wrong?
        
        /*  
            Each user should be assigned a ID according to its vector index in state.
        */
    }
}

#[derive(Debug)]
pub struct User {
    // id: usize, //should be added
    name: String,
    list: Vec<todo>,
}

#[derive(Debug)]
#[allow(non_camel_case_types)] //remove this
struct todo {
    sno: u8,
    task: String,
    status: bool,
}

impl todo {
    // this runs in `post_list()` method.
    fn create(state: &State, owner: &str) -> Self {
        
        // requests to know what task
        let mut todo = String::new();
        print!("Task:\n?>\t");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut todo).unwrap();

        
        // creating temp_var & setting default values
        let mut count_max: u8 = 0;
        let mut newuser = true;


        /* printing &state.0 to understand what it is */
        println!("[DEBUG] > {:#?}", &state);

        /* CONCLUSION:
            Wrong implementation.
            In each post, Creating different `User` in `Vec<User>` found in `State`, even when names are same.

            [structure]:
            State >> users
            user >> todos
            todo >> sno, task, status

            [wrong implementation]:
            State >> Users keeps on adding in state with each post.
            user >> todos remain todo. post doesn't pushes to relevant place (i.e todos). It creates a new `User` and pushes directly to the `State`.

            [rectification]:
            if new user only then create a user.
            if old user, access its list and push to it instead.
            the list is known as `todos`.

            [ANSWER]:
            `State` is actually a tuple struct. So anytime you refer to a tuple struct you use `StructName.0`.
            Hence in order to refer to `Vec<Users>` in
            `State(Vec<Users>)`, I will have to use state.0;
            where `state` is the instance of `State(Vec<User>)`

         */

        for i in &state.0 { // we're finding our `user` in the state.
            if i.name == owner.to_owned() { //owner is type `str` so using `.to_owned()` to essentially convert it to `String`.

                /* notice:
                    in `post_list()` we get the name as input.
                    in `create()` we find the associated list.

                */

                newuser = false; //if user found then newuser=false
                for j in &i.list { //in this list, we're going into his todo for some reason
                    if count_max < j.sno { //we use `count_max` to essentially calculate what `sno.` will be the list on.
                        count_max = j.sno
                    }

                    /* 
                     double loop isn't really needed here.
                     I can add `sno.` to user column instead and renaming it has `todo_count`.
                     Then I whenever I push to list, I can update that count without going spinning a nested loop.

                     If I add delete functionality, I can just reduce the count.
                     */

                }
            } 
        }
        count_max += 1; //oh god

        
        print!("list created!:\nEntry ~ {count_max}.{todo}");

        if newuser == true { println!("\t[info: new user detected!]"); }
        println!();

        // returning task
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
        /* 
            It takes `state` as parameter.
            So it see what all users and their todolist are there.
        */


        // requests username to recognize which todolist to post to
        let mut author = String::new();
        print!("Who's list?: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut author).unwrap();
        author = author.trim().to_string();

        // once it gets the list, it initializes the create task function.
        let task = todo::create(server, &author);
        /* unneccessarily state is being repeatedly passed! */

        // combine to make a list || make an instance && store it in State; // ⬅️ old comment
        let created_list = User { name: author, list: vec![task] }; //oh great, instead of pushing task into the list, I create another list of task (vector of task).
        State::push_it(server, created_list) //and then I push it onto the server for some reason
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

        // get info // ⬅️ old comment

            // request name to recognize which list (using this multiple times now so can make an abstract function out of this)
            print!("which user?: ");
            let mut user = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut user).unwrap();
            user = user.trim().to_string();

            // TASK // ⬅️ old comment
            print!("target no.?: "); //here i basically ask the serial number
            let mut sequence = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut sequence).unwrap();
            let serialnombar: u8 = sequence.trim().parse().expect("Failed to parse number"); //nois spalleeng.
 
         // access & edit // ⬅️ old comment
        for i in server.0.iter_mut() { //oh god! I don't know what the fuck is this. THIS.IS.HELL.
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
                    } //i give up
                }
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Username INVALID 🤨")));
            }
        }
        
        Ok(()) //yeah i'm basically gonna rewrite the whole thing now.

    }

    pub fn show_everything(server: &State) {
        println!("all users:\n{:?}", server.0); //the only sane piece of code
    }
}

/* how to get a task? search thru all_users for i in self.0 where i.name == required, then get the task */ // ⬅️ old comment