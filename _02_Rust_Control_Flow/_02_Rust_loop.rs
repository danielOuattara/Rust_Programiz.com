/* Rust loop
============= */

pub fn main() {
    /*
        In programming, a loop is used to execute a code
        block multiple times. For example, to print a number
        100 times, we use a loop instead of writing the
        print statement repeatedly.

        In Rust, you can use three different keywords to
        execute a code block multiple times:

        - loop
        - while
        - for

        Loop Expression
        ================

        In Rust, we use the loop expression to indefinitely
        execute a block of code. If we use a loop, the code
        execution inside of the loop code block doesn't stop
        and runs forever.

        The syntax of the loop expression is:

        loop {
            // code to execute
        }
    */

    // loop expression
    // loop {
    //     println!("Loop forever!");
    // }

    /*Output

    Loop forever!
    Loop forever!
    Loop forever!
    .
    .
    .

    This example code will print "Loop forever!" indefinitely
    unless the user terminates the program. Since the loop
    runs forever, it is also known as an infinite loop.


    Terminating Loop in Rust
    ========================

    We use the break keyword to terminate a loop: */

    // initiate an infinite loop
    loop {
        println!("Loop forever!");

        // stop infinite loop
        break;
    }

    /*
    Here, the 'break' keyword terminates the loop. That is
    why the println! macro is executed only once.

    Note: In Rust, we often use a loop and break together.

    Example: Print First 10 Natural Numbers using Loop */

    let mut number = 0;

    // infinite loop starts here
    loop {
        number += 1;
        println!("{}", number);

        if number >= 10 {
            // exit the loop
            break;
        }
    }

    /*
    In the above example, we have used a loop expression to
    print the natural numbers. Here, the initial value of the
    number variable is 0.

    To learn more about the break keyword, visit Rust break and
    continue. */
}
