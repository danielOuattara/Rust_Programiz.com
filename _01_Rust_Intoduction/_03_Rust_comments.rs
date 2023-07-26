/*
Rust Comments
===============

In computer programming, comments are lines of text used
to describe the purpose of code. For example,

// entry point of the program
fn main() {
    // print text on the screen
    println!("Hello, World!");
}

Here comments are :
- // entry point of the program
- // print text on to the screen are comments.

Comments are completely ignored and not evaluated during
code execution. Ideally, a comment should allow the reader
to understand what a piece of code is doing.


Types of Comments
==================

There are two important types of comments in Rust:

    // - Line Comment
    /*...*/ - Block Comment


Line Comment in Rust
=====================
In Rust, we use two forward slashes, //, to create a line
comment. For example,

fn main () {
    // declare a variable
    let x = 1;
    println!("x = {}", x);
}


Here, // declare a variable is a line comment.
The comment extends up to the end of the line and is also
known as single-line comments.

We can also use line comments in the same line as the code.
For example,

fn main() {
    let x = 1;    // declare a variable
    println!("x = {}", x);
}

Here, // declare a variable is also a line comment placed at
the end of the line containing code.


Block Comment in Rust
======================
In Rust, we use the symbol /*...*/ to denote the block comment.
It starts with /* and ends with */. For example,

fn main() {
    /*
    declare a variable
    and assign value to it
    */
    let x = 1;
    println!("x = {}", x);
}

Here,

/*
declare a variable
and assign value to it
*/

is a block comment. You can see the block comment extends
for multiple lines. Hence, it is also known as multi-line
comments.

We can also create multi-line comments using multiple line
comments.
For example,

fn main() {
    // declare a variable
    // and assign value to it
    let x = 1;
    println!("x = {}", x);
}

Here, we have used two single-line comments:
// declare a variable
and
// and assign value to it instead of a multi-line comment.

Note: In the Rust ecosystem, line comments are preferred
      over block comments.


Disable Parts of Code Using Rust Comments
==========================================
Comments are also useful for temporarily disabling chunks
of code.Let's see an example:

fn main() {
    let x = 1;
    let y = 2;
    let z = 3;
    println!("z = {}", z);
}

This piece of code will throw a warning because both x and
y variables are unused. Instead of completely removing
these declarations, we can comment them.

fn main() {
    /*
    temporarily disable x and y variable declarations.
    let x = 1;
    let y = 2;
    */

    let z = 3;
    println!("z = {}", z);
}


Now, only the code outside of the block comment will be
evaluated. We have temporarily disabled part of the code
that was triggering a warning.

Code comments can be helpful in these scenarios.


Frequently Asked Questions
===========================

- What are the advantages of Comments in Rust ?
-----------------------------------------------

Here are some of the major benefits of using comments:

1. Make Code Easier to Understand:
   - Writing comments make our code readable and easier for
   future reference.

2. Using Comments for debugging:
   - Comments can be used to
   ignore a block of code that causes an error during
   debugging.

3. Using comments for efficient collaboration:
   - Comments can help peer developers to understand each
   other's code better.



How to create better comments ?
--------------------------------

Writing better comments is one of the important parts of
being a Rust developer. Our code will be used by multiple
developers in multiple projects. So, a well-written
comment is useful to provide context to fellow programmers
while reading our code.

Here are a few ways to improve code comments:

- Comments shouldn't only explain what the code does,
  instead, our code should be self-explanatory, and
  comments should provide context around the code.

- Try to use short and precise comments.
- Don't overuse comments.
- Don't use redundant comments.
*/
