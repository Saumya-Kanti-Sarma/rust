Rust is a modern, system-level programming language designed to provide high performance, reliability, and safety, specifically by preventing memory errors without using a garbage collector.  Developed originally by Mozilla employee Graydon Hoare in 2006 and officially released in 2010, it became a stable language with Rust 1.0 in May 2015

Key characteristics of Rust include:

Memory Safety: It uses an ownership and borrowing system checked at compile time to prevent issues like null pointer dereferences and data races. 
Performance: It offers zero-cost abstractions, allowing developers to write high-level code that compiles to efficient, low-level machine code comparable to C or C++. 
Concurrency: It enables safe parallel programming, making it ideal for multi-core processors and complex backend systems. 
Ecosystem: It is widely used for web development, operating systems, embedded systems, blockchain, and game engines. 
Rust is often chosen as a safer, faster alternative to C and C++, balancing the control of low-level languages with the ease of high-level abstractions. 

Difference between Rust and C\C++
The main difference is that Rust is "safe by default".
In Rust, memory access is automatically checked by the compiler, which helps you avoid common bugs like crashes, leaks, and unsafe memory behavior.
In C and C++, you must manually manage memory, which can lead to errors if you are not careful

Create a new Project with rust:
cargo new my_project
This creates a folder called my_project with the following files:

Cargo.toml: Project settings
src/main.rs: Main Rust file
The main.rs file contains this default code:

fn main() {
  println!("Hello, world!");
}

cd my_project
cargo run

Example explained
Line 1: fn main() is something that always appears at the beginning of every Rust program. main() is called a function, and any code inside its curly brackets {} will be executed.

Line 2: println!() is a macro, used to output/print text to the screen. In our example it will output "Hello World!". To end the code, you must remember a semicolon (;).

What is a macro?

A macro is like a function, but with an exclamation mark (!) after it. Don't worry about the terminology for now. For now, just know that macros are similar to functions (they execute things), but they do not always follow the same rules as functions. You will learn more about macros later.

Good to know: The Rust compiler ignores white spaces.

The code above could also been written as:
fn main(){println!("Hello World!");}

However, multiple lines and indentation makes the code more readable.