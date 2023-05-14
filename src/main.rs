pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    let user = whoami::username();

    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");

    repl::start()
}
