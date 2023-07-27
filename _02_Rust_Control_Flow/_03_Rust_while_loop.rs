/* Rust while loop
=================== */

fn main() {
    /*
    We use the while loop to execute a code block till the
    condition is true. The syntax for the while expression is:

    while condition {
        // code block
    }

    // code block outside while loop

    Here, the while loop evaluates the condition before
    proceeding further.

    If the condition evaluates to:

    - true, the code block inside the while loop is executed
    and the condition is evaluated again
    - false, the loop terminates and the code block outside
    the while loop is executed */

    let mut counter = 1;

    // usage of while loop
    while counter < 6 {
        println!("{}", counter);

        counter += 1;
    }
    /*
    In the example above, we have a condition:

    while counter < 6 {
        // code block
    }

    Here, the loop keeps running till the counter variable
    is less than 6. Inside the loop, we are increasing the
    value of the counter by 1.

    After 5th iteration, the value of counter will be 6, so
    the condition, counter < 6 becomes false and the loop is
    terminated.

    Note: while expressions are generally used in conjunction
    with counter variables that help exit the loop after certain
    conditions.

    Working of while Expression in Rust
    ====================================


    Infinite while Loop
    ====================

    You can write a loop that never ends using the while
    expression: */

    let _counter = 1;

    // while _counter < 6 {
    //     println!("Loop forever!");
    // }

    /*
    This example code will print "Loop forever!" indefinitely
    because the condition counter < 6 always evaluates to true.
    It is because we never increase the value of the counter
    variable inside the loop. Thus, this program will run until
    the user terminates the program.

    Note: You can use the break keyword to terminate any kind of
    loop in Rust.

    Example: Multiplication Table Using while Loop */

    // variable to print multiplication table for
    let i = 2;

    // counter variable that starts at 1
    let mut j = 0;

    // while loop that runs for 10 iterations
    while j <= 10 {
        // multiply i and j
        let multi = i * j;

        // print multiplication result on each iteration
        println!("{} * {} = {}", i, j, multi);

        // increase value of counter variable j
        j += 1;
    }

    /*
    Nested while Loop
    ===================

    We can use a while loop inside the body of another while
    loop. This is known as a nested while loop. A nested
    while loop looks like:

    while outer_condition {
        // outer code block 1

        while inner_condition {
            // inner code block
        }

        // outer code block 2
    }

    Let's print a pattern using a nested while loop */

    // outer loop counter
    let mut i = 1;

    // outer loop
    while i <= 5 {
        // inner loop counter
        let mut j = i;

        // inner loop
        while j <= 5 {
            print!("*");

            // increase inner loop counter
            j += 1;
        }

        println!("");

        // increase outer loop counter
        i += 1;
    }

    /*
    In the above example,

        - The outer while loop iterates 5 times

        - The inner while loop inside of the outer while loop
          also iterates 5 times

        - The inner while loop prints an asterisk(*) → print!(*)
          on every iteration

        - The inner while loop stops when the counter variable j
          reaches to 6 as the inner condition evaluates to false

        - The outer while loop prints a new line → println!("")
          on every iteration and goes to the next iteration which
          will initiate the inner while loop again

        - The outer while loop stops when the counter variable i
          reaches to 6 as the outer condition evaluates to false

     */
}
