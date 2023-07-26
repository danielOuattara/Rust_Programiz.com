/*
Rust Hello World Program
=========================

A "Hello, World!" program prints the text Hello, World! to
the screen. Since it's a very simple program, it's often used
as an ice breaker to learn a new programming language.

Let's explore how the "Hello, World!" program works in Rust.


Rust "Hello, World!" Program
=============================*/
fn main() {
    println!("Hello, World!");
}

/*
Output: Hello, World!

As you can see, we have successfully printed the text Hello, World!
on the screen.


Working: Rust Hello World Program
==================================

Here are the different parts of the program above:

1. The main() Function
-----------------------

fn main() {
}

This is the main() function which acts as an entry point of every
Rust program. It is always the first code that runs in every Rust
program.

The body of the function is wrapped inside curly brackets, {}.
We will learn more about functions in later chapters.


2. Print Statement
-------------------

println!("Hello, World!");

We use the println! macro to print text to the screen.
The "Hello, World!" string is an argument to println!().
Finally, we end the line with a ';' which indicates that
the expression is over.

We will learn more about Rust macros in upcoming tutorials.

*/
