## Programming a Guessing Game
- program will generate a random integer between 1 and 100
- it will then prompt the player to enter a guess
- after guess is entered, program will indicate whether guess is too low or too high
- if guess correct, the game will print a congratulatory message and exit

---
### Setting Up a New Project
```console
$ cargo new guessing_game
$ cd guessing_game
```
- `cargo new` takes name of the project (`guessing_game`)
- `cd guessing_game` changes to the project directory

ㅤ 
Filename: [[Projects/guessing_game/Cargo.toml|Cargo.toml]]
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

ㅤ 
Filename: [[Projects/hello_cargo/src/main.rs|src/mail.rs]]
```rust
fn main() {
    println!("Hello, world!");
}
```

ㅤ 
- compile "Hello, world!" program
```console
$ cargo run
```
- `run` command comes in handy when rapidly iterating on a project, quickly testing each iteration before moving on to the next one

---
### Processing a Guess
- first part: ask for user input, process input, check that the input is in expected form
- allow player to input a guess

Filename: [[Projects/hello_cargo/src/main.rs|src/mail.rs]]
```rust
// Listing 2-1: Code that gets a guess from the user and prints it
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
ㅤ 

- to obtain user input and then print result as an output
	- will need `io` input/output library into scope
		- from standard library, `std`
```rust
use std::io;
```
- Rust has a set of items defined in the standard library that it brings into the scope of every program, called *prelude*
	- [standard library documentation](https://doc.rust-lang.org/std/prelude/index.html)
- if a type isn't in the prelude, will need to bring that type into scope explicitly with a `use` statement
- `std::io` library provides a number of features, including the ability to accept user input

ㅤ 
- main function (entry point into the program)
```rust
fn main() {
```
- `fn`
	- declares new function
- parentheses `()`
	- indicates that there are no parameters
- curly bracket `{`
	- starts the body of the function

ㅤ
- `println!`
	- macro that prints a string to the screen
```rust
	println!("Guess the number!");
	println!("Please input your guess.");
```
- code prints a prompt stating what the game is and requesting input from user

---
### Storing Values with Variables
- *variable*
```rust
    let mut guess = String::new();
```
- `let` statement to create variable
```rust
let apples = 5;
```
- creates a new variable named `apples` and binds it to the value 5
- in Rust, variables are *immutable* by default
	- once variable is given a value, value won't change
- to make variable mutable, add `mut` before variable name
```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
> Note: `//` syntax starts a comment that continues until the end of the line. Rust ignores everything in comments

- `let mut guess`
	- introduces a mutable variable named `guess`
- `=`
	- tells Rust something is bound to the variable
- right of `=`
	- value that `guess` is bound to
		- result of calling `String::new`
			- a function that returns a new instance of a `String`
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
	- a string type provided by the standard library
	- UTF-8 encoded bit of text
- `::` syntax in the `::new` line
	- `new` is an associated function of the `String` type
	- *associated function*
		- function that's implemented on a type, in this case, `String`
	- `new`
		- function that creates a new, empty string
		- common name for a function that makes a new value of some kind
- `let mut guess = String::new();`
	- created a mutable variable that is currently bound to a new, empty instance of a `String`

---
### Receiving User Input
- recall: `use std:io;`
	- input/output functionality from standard library
	- can call `stdin` function from the `io` module to handle user input
```rust
    io::stdin()
	    .read_line(&mut guess)
```
- if `io` library wasn't imported with `std::io;`
	- can still use function using `std::io::stdin`
		- `stdin`
			- returns instance of `std::io::Stin`, a type that represents a handle to the standard input of terminal

- `.read_line(&mut guess)`
	- calls the `read_line` method on the standard input handle to get input from user
	- passing `&mut guess` as the argument to `read_line`
		- tells it what string to store user input in
	- `read_line`
		- take user input into standard input, append that into string without overwriting, pass string as argument
		- string argument needs to be mutable so method can change string's context
- `&`
	- indicates that argument is a *reference*
		- gives a way to let multiple parts of code access one piece of data without needing to copy that data in memory multiple times
		- references are immutable by default
			- will need to write `&mut guess` rather than `&guess` to make it mutable

---
### Handling Potential Failure with Result
```rust
        .expect("Failed to read line");
```

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```
- introduce a newline and other whitespace to help break up long lines when calling method with `.method_name()`

- `read_line` returns user input as a `Result` value
	- [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) is an *enumeration* (*enum*), a type that can be in one of multiple possible states, a *variant*
- `Result`
	- variants are `Ok` and `Err`
		- `Ok`
			- indicates operation was successful
			- contains the successfully generated value
		- `Err`
			- indicates operation failed
			- contains information about how or why operation failed
	- values of `Result` type have methods defined on them
		- an instance of `Result` has an `expect` method that can be called
			- if instance is `Err`
				- likely be the result of an error coming from underlying operating system
			- if instance is `Ok`
				- `expect` will take the return value that `Ok` is holding and return just that value to be used
			- if `expect` is not called
				- warning
					- `Result` value returned from `read_line` haven't been used
					- indicates program hasn't handled a possible error
					- suppress warning by writing error-handling code

### Printing Values with `println!` Placeholders
```rust
    println!("You guessed: {}", guess);
```
- prints the string that contains user input
- `{}` is a placeholder
	- when printing value of a variable, variable name can go inside curly brackets
	- when printing result of evaluated expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in same order

- printing variable and the result of an expression in one call to `println!`
```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

```rust
x = 5 and y + 2 = 12
```

### Testing the First Part
```console
$ cargo run
```
- done: getting input from keyboard then printing it
