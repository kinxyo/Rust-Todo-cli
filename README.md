> [!IMPORTANT]
> Work In Progress.

# Interactive CLI Todo App

~~A simple todo app written in Rust because I just wanted to write my own state management system from scratch.~~

Initial motivation was to create a simple todo list but then out of nowhere, I was inspired to add more features to make it a full-fledged interactive CLI app.

## ¿ Why `Rust🦀` ?

Because I love this crab language.

## System Design

There are 3 files in the codebase:-

### 1. _Todo.rs_

This file incorporates all the primary backend functions.

#### Structs

The backend can be divided into 2 layers, following their responsible structs.

- **Implementation Layer**
   * `State` struct is responsible for this layer.
   * It is essentially a tuple struct that holds a HashMap containing the `User` object.
   * Alternatively, `State` can also be referred to as the "_server_" since it fulfills all the backend functions. This designation is apt because an instance of `State` encompasses and manages all the data.
   * In the HashMap, the 'key' is the 'username' (which is a `String`) and the value is the `User` object. Now, I know that `String` is slower to hash than any number-type however, this is a smaller application so I don't think it would have be neccessary. Creating a `user_id` would only complicate this simple project.
- **Abstraction Layer**
   * `User` struct is responsible for this layer.
   * It serves as a mere abstraction layer as the `main` function exclusively engages with the `User` struct, treating it as an interface to access all the backend functions.

#### Facade

- `User::welcome()` creates an instance `State` and loads previous data into it (if present), otherwise returns a fresh instance.

- `User::handle()` returns 'username' of the logged-in user. This username, as state before, is the key of the HashMap so via it I can access the `User` object of the logged-in user without passing a mutable reference to the `User` object everywhere (like I was doing before).

- `User::task()` Takes input for todo-list and call upon other backend functions to accordingly manipulate the state.

- `User::view()` returns a vector of all the tasks of the logged-in user.

(with more to come)

#### Error Handling & Bug Reporting

##### Error Handling

There are a lot of methods that return a `Result`. It's rather hectic to individually handle all of them (actually, I did just that and ended up making code very unreadable).
Hence, I used something that I like to call **_Wrapping Function_**.

Basically, I apply `?` operator on returning methods, and then I wrap them in a function that returns `Result` and then I apply `match` case on that function. This way, I can keep the code clean and also handle the errors.

##### Bugs & Panics

There are some errors that are never going to happen under normal circumstances, but if supposedly they do then there really is no way to keep the program running after that. In such extremely rare cases, I would prefer to crash program. Therefore, I have deliberately let such errors panic. Also, it would be more appropriate to call them **bug** instead.
For example :-

```rust
let file = std::fs::OpenOptions::new().create(true).append(true).read(true).open("db.json").expect("Database Error");
```

I want the program to crash here because if `db.json` is not created (when not present) then there will be no 'State Preservation', the program will be of no use. However, this is very unlikely to happen & probably only because of permission issues. There is nothing I can do to get permission from the app itself, this solely lies on user's end.

> What if it crashes for some other reason?

That's a valid concer, hence, to not let any crash go in vain, I have implemented a **bug reporting system** that logs the error in a file. The user can see the log and search the error in the documentation. In the event that the documentation lacks information on the specific error, the user then has a choice to send this file to the developer (me) to help them solve this issue.

Althought this project is not so serious that this will be actually needed but I just wanted to implement this feature to learn how to do it.

### 2. _functions.rs_

There were some methods that were called being multiple times in multiple files but with some sort of tweaking going-on, so I wrapped them in functions to streamline the process.

### 3. _main.rs_

Entry point of the program where all code converges. That might have been a poetic description but it conveys its essence, and that's all there is to it.

## Crates Used

1. Serde (for preserving `State` in JSON format)
2. Dialoguer (for making CLI interactive)

## Features left to implement (TODO)

- [x] Refactor Code
- [x] Error log
- [x] Persistance of state
- [x] Panic if unable to create or load `(db + error).json`.
- [x] Add support for selecting items using arrow key (Interactive Terminal).
- [x] Add `logout` functionality.
- [x] Show Users on Welcome Page
- [ ] Handle case where `password` is wrong.
- [ ] Add support of `remember user`.
- [ ] Limit `username` to 10 characters.
- [ ] Limit user accounts to 5 people.
- [ ] Add datetime to error log.
- [ ] Add `help` command.
- [ ] Add a `delete db.json` function when 'data is corrupted' error occurs.
- [ ] Add full-fledged Authentication System.
- [ ] Update to proper Argument Parser.
- [ ] `Complete Task` Functionality
- [ ] `Remove User` Functionality
- [ ] `Delete a Task` Functionality
- [ ] **Styling**
