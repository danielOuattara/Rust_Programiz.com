/*
Rust function
=============== */

fn main() {
    /*
    Functions are reusable blocks of code that perform a specific
    task. For example, if we want to create a program to add two
    numbers, then we can create a Rust function to add numbers.
    Now, we can reuse this same function whenever we add two numbers.

    Creating a function in Rust helps divide our code into smaller
    blocks and makes our code look cleaner and easier to understand.

    Not only in Rust, but functions are also one of the core building
    blocks of any programming language.


    Define a Function in Rust
    ==========================

    In Rust, we use the 'fn' keyword to define a function.
    The syntax of a function is:

    fn function_name(arguments) {
        // code
    }

    Let's see an example.

    fn greet() {
        // code
    }

    Here,

    - fn - keyword used to create a function in Rust
    - greet() - name of the function
    - // code - function body
    - { } - start and end of the function body

    Now let's complete the greet() function to print "Hello, World!".
    */

    // define a function
    fn greet() {
        println!("Hello, World!");
    }

    /*
    When we run this code, we will not get any output. This is
    because here we are just defining a function. To execute a
    function, we need to call it.


    Calling a Function in Rust
    ===========================

    We use the name of the function and parentheses () to call
    a function. */

    // call a function
    greet(); // Hello, World!

    /*
    Here, we have created a greet() function that prints
    "Hello, World!" on the screen. Notice that we are defining and
    calling the function from inside main() function.


    main() Function in Rust
    =======================

    If you look carefully, you can see the syntax of main() looks
    similar to a function.

    fn main() {
        // function call
        greet();
    }

    In Rust, main() is also a function known as a built-in function
    that has a special meaning. It is the entry point (start) of every
    Rust program.

    Note: Rust code uses a small case as the convention for defining
          a function name. An extended function name with multiple
          words will have underscores in between words.

    Example: Function to Add Two Numbers in Rust
    */

    // function to add two numbers
    fn add1() {
        let a = 5;
        let b = 10;

        let sum = a + b;

        println!("Sum of a and b = {}", sum); // Sum of a and b = 15
    }
    add1();

    /*
    In the above example, we have created a function named add().
    The function adds two numbers and prints the sum.


    Function Parameters in Rust
    ============================

    From the definition, we know that a function should be reusable.
    However, the add() function in our previous example can only be
    used to perform the addition of 5 and 10. So this function is not
    dynamic to be reused.

    To deal with this and make our functions more dynamic, we can
    create functions that accept external values. These external values
    are called function parameters.

    Here's how we can create a function with parameters. */

    // function with parameters
    fn add2(a: i32, b: i32) {
        let sum = a + b;

        println!("Sum of a and b = {}", sum);
    }

    add2(3, 16);

    /*
    Here,

    - a and b are function parameters
    - i32 is the data type of parameters

    To call this function, we should provide some value during
    the function call.

    Here, 2 and 11 are known as function arguments that are passed
    to the add function.


    Function with Return Value in Rust
    ===================================

    In the last example, we computed the sum of two numbers and
    printed the result inside the function. However, we can also
    return the result from the function and use it anywhere in
    our program.

    Here's how we can create a function in Rust that returns a
    value.
    */

    // define an add function that takes in two parameters with
    // a return type

    fn add3(a: i32, b: i32) -> i32 {
        let sum = a + b;

        return sum; // return a value from the function
    }

    /*
    Here, -> i32 before the opening curly bracket '{' indicates
    the function's return type. In this case, the function will
    return an i32 value.

    We have then used the return keyword to return the sum
    variable from the function.

    The function returns the value to the place from where it
    is called, so the returned value needs to be stored somewhere.
    */

    // store the returned value in a variable
    let sum3 = add3(3, 5);
    println!("sum3 = {sum3}");

    /*
    In the above example, when we reach the return statement in
    the add3 function, it returns the sum of addition. The
    returned value is stored in the sum variable inside main().


    Frequently Asked Questions
    ==========================

    How can we return a value from a function with a Rust expression ?
    ------------------------------------------------------------------

    A function that ends with an expression will return the value
    of the expression. It means that we don't have to use the
    return keyword to return a value from a function.
    For example: */

    fn addition1(a: i32, b: i32) -> i32 {
        a + b
    }

    let summation1 = addition1(1, 2);
    println!("Sum = {}", summation1); // Sum = 3

    /*
    Note: Notice a + b doesn't end with a semicolon in the function
    body. Expressions do not include ending semicolons, but statements
    do.


    How can we return multiple values from a function in Rust ?
    -----------------------------------------------------------

    We can return multiple values from a function using tuples.
    For example*/

    fn add_and_sub(a: i32, b: i32) -> (i32, i32) {
        return (a + b, a - b);
    }

    let (sum, diff) = add_and_sub(4, 1);
    println!("Sum = {}, Difference = {}", sum, diff); // Sum = 5, Difference = 3

    /*
    Here, the function's return type is a tuple (i32, i32).


    How to pass by reference in Rust ?
    ----------------------------------

    We can use 'pass by reference' to pass a pointer of the
    variable instead of the actual variable.

    For example, */

    let word = String::from("hello");

    // passing reference of word variable
    let len = calculate_length(&word);
    println!("The length of '{}' is {}.", word, len) // The length of 'hello' is 5

    fn calculate_length(s: &String) -> usize {
        return s.len();
    }

    /*
    Here, we pass the word variable as a reference to the function 
    calculate_length() with '&word'.


    What are the advantages of functions in Rust ?
    ----------------------------------------------
    Functions are building blocks of Rust programming language 
    and come with many advantages. Some of them are:

    - Functions divide our code into smaller, reusable blocks.
    
    - Functions help make our programs easier to read and easier 
      to debug.
    
    - Functions make our program modular, easier to change and help 
      in reducing code duplication.
     */
}
