# Rust Todo-cli

A feature-full Rust-based Todo app that installs and runs on cli because I just want to write my own state management system from scratch.

![image](https://github.com/kinxyo/Rust-Todo-cli/assets/90744941/e8a9b0ca-2059-4315-b510-7c939620df13)

## TODO

- [x] Refactor Code
- [x] Error log
- [x] Persistance of state
- [x] Panic if unable to create or load `(db + error).json`.
- [ ] Show Users on Welcome Page (if any)
- [ ] Add full-fledged Authentication System.
- [ ] Add support for selecting items using arrow key
- [ ] Update to proper Argument Parser.
- [ ] Updating Error log to Server (at exit command perhaps).
- [ ] `Remove User` Functionality
- [ ] `Delete a Task` Functionality

------------

## System Design

There are 2 main structs in this code: `State` and `User`.

- `State` can also be conversely called as "_server_" as it performs **all** the backend functions. This is because it's its instance that's holding everything (all the data).

- `User` is merely just an abstraction layer. The `main` function only interacts with `User` struct as it acts like an **API** to all the backend functions.

```rust
// Post This In Readme.md ⬇️

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

/* Wrapper functions: 
    In code there are a lot of methods return a `Result`. 
    I can not go and individually apply `match` case to them (achulli, I did just that but it made code very ugly)
    Hence, I use `?` operator on them,
    and then I wrap them in a function that returns `Result` and then I apply `match` case on that function.
    This way, I can keep the code clean and also handle the errors.
*/

```

What I asked:
> In order to create my own state management system in Rust, I have created two structs -- `State` & `User`. I have also created their respective functions. I create their instance in the beginning of the program and pass their mutable reference onto their respective functions throughout the program. I also preserve the state(data) in a `.json` file, and retrieve it when the user wants to continue their session. Is this the right approach?

What _bing_ replied:
> Your approach of creating two structs, state and user, and their respective functions, and then passing their mutable reference throughout the program is a good way to manage state in Rust.
> Preserving the state data in a .json file and retrieving it when the user wants to continue their session is also a good way to persist the state data between sessions.

What _github copilot_ replied:
> Yes, if you want to modify `state` or `user` within the `User::task` or `User::all` methods, you would need to pass mutable references to these methods.

## `RefCell` Example

```rust
use std::cell::RefCell;

struct MyStruct {
    value: RefCell<i32>,
}

impl MyStruct {
    fn new(value: i32) -> MyStruct { //ill have to change my create user code and struct field
        MyStruct {
            value: RefCell::new(value),
        }
    }

    fn increment(&self) {
        let mut value = self.value.borrow_mut();
        *value += 1;
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

fn main() {
    let my_struct = MyStruct::new(42);
    my_struct.increment();
    println!("Value: {}", my_struct.get_value());
}
```

------------
> [!IMPORTANT]
> Work In Progress.
