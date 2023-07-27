/* Rust for Loop
================= */

fn main() {
    /*
    The for loop in Rust is used to iterate a range of numbers.
    The syntax of for loop is:

    for variable in lower_bound_number..upper_bound_number {
        // code block
    }
    */

    // usage of for loop
    for i in 1..6 {
        println!("{}", i);
    }

    /*
    In this example, we print numbers 1 to 5 using the for syntax.

    Here:
    - for : is the keyword to start any for loop
    - i   : is known as the loop variable and should be a valid variable name
    - in  : is the keyword used to iterate over a series of values with for
    - 1..6: is known as an Iterator where 1 is the lower bound and 6 is the upper bound. This yields values from 1 (inclusive) to 6 (exclusive) in steps of one.

    Note: The for loop is also known as a for-in loop because of its syntax.

    Example: Sum of First 10 Natural Numbers using for Loop
    --------------------------------------------------------*/

    let mut sum = 0;

    // for loop to iterate over first 10 natural numbers
    for i in 1..11 {
        sum += i;
    }

    println!("Sum: {}", sum);

    /*
    Here, we loop over the iterator 1..11, which yields values
    from 1 to 10. A 'sum' variable is created to sum all the
    values in each iteration. Finally, we print the sum of all
    the values.

    Note: The 1..11 syntax is also known as a range notation or
    range operator used to create Iterators in Rust.

    To learn more about iterators, visit Rust Iterator.

    Frequently Asked Questions
    ============================

    Does Rust have a "C-style" for loop ?
    -------------------------------------
    for (i = 0; i < 10; i++) {
        printf("%d\n", i);
    }

    By design, Rust does not have the "C-style" for loop.
    The "C-style" for loop has four major components:
    - initialization,
    - condition,
    - update expression
    - and a loop body.

    With this syntax, the user needs to control and define
    every part of the code which is complicated and error prone.


    How to use range notation that is inclusive on both ends ?
    ----------------------------------------------------------

    The range notation ..= can be used for an inclusive range on
    both ends. For example, 1..=5 yields values from 1 (inclusive)
    to 5 (inclusive) in steps of one.*/

    // for loop with inclusive range notation
    for i in 1..=5 {
        println!("{}", i);
    }

    /*
    How to use for to loop over an array or a list ?
    ------------------------------------------------

    We can loop through an array using the same for..in syntax*/

    let fruits = ["Apple", "Orange", "Banana"];

    // for loop to iterate through items in an array
    for fruit in fruits {
        print!("{}, ", fruit);
    }

    /*To learn more about arrays, visit Rust Array. */
}
