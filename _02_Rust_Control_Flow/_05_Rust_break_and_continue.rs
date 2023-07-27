/* Rust break and continue
=========================== */

fn main() {
    /*
    A loop executes a block of code multiple times. However,
    sometimes we might need to alter the flow of a loop by
    terminating its execution or skipping an iteration.

    In such cases, we use the Rust 'break' and 'continue' to
    alter the normal execution of loops:

    - break    : terminates the loop
    - continue : skips the current iteration of the loop and moves on to the next


    Rust break
    ==========

    In Rust, we use the 'break' keyword to terminate the
    execution of any loop.*/

    // while n < 10 {
    //    break;
    // }

    /*
    Here, the while loop will end whenever it encounters the
    break keyword, irrespective of the loop condition (n < 10) */

    let mut number = 0;

    // loop starts here
    loop {
        println!("{}", number);
        number += 1;
        if number > 5 {
            // condition to exit the loop
            break;
        }
    }

    /*
    This program prints the first five natural numbers using
    a loop expression and a break keyword. Notice the use of
    the break keyword.

    When the number variable, which is incremented by 1 in
    each iteration, reaches 6, the if condition evaluates to
    false, and we exit the loop using the break keyword.

    Note: It is up to the user to define the condition when
    the loop exits, or else the loop might run forever.

    You can use the break keyword with while or for loops in
    a similar pattern.


    Rust break with Nested Loops
    ============================= */

    let mut i = 1;

    // start of outer loop
    while i <= 5 {
        let mut j = 1;

        // start of inner loop
        while j <= 5 {
            println!("i = {i} , j = {j} ");

            // condition to exit the inner loop
            if j == 3 {
                // terminate the inner loop
                break;
            }

            j += 1;
        }

        println!("");

        i += 1;
    }

    /*
    In the above example, we have used the break keyword in
    the body of the inner while loop.

    if j == 3 {
        // terminate the inner loop
        break;
    }

    When the value of the counter variable j reaches 3, the
    inner while loop terminates. As a result, we only see
    three asterisks (***) printed on every line of the screen.

    Rust continue
    =============

    In Rust, we use the continue statement to skip the current
    iteration of any loop and move to the next iteration;

    while n < 10 {
        if n == 5 {
            continue;
        }
    }

    Here, the while loop will skip the current iteration when
    it encounters the continue keyword irrespective of the
    loop condition (n<10). */

    let mut number = 0;

    while number < 5 {
        number += 1;

        // condition to skip the iteration
        if number == 3 {
            continue;
        }

        println!("{}", number);
    }

    /*
    In this example, we use the while expression to print
    natural numbers. Notice the use of the continue keyword,

    if number == 3 {
        continue;
    }

    Here, we skip the iteration when the number variable is
    equals 3. As a result, we don't see 3 in the output.


    Rust continue with Nested Loops
    ================================= */

    let mut i = 1;

    // start of outer loop
    while i <= 5 {
        let mut j = 1;

        // start of inner loop
        while j <= 5 {
            j += 1;

            // condition to skip iteration of the inner loop
            if j == 3 {
                // move to the next iteration of the inner loop
                continue;
            }

            println!("i = {i} , j = {j} ");
        }

        println!("");

        i += 1;
    }

    /*
    Here, we have used the continue keyword to skip an
    iteration of the inner while loop.

    if j == 3 {
        // move to the next iteration of the inner loop
        continue;
    }

    When the value of the counter variable j reaches 3,
    we skip the current inner while iteration and the print!("*")
    statement is skipped. As a result, we only see four
    asterisks (****) printed on every line of the screen.
    break and continue with loop

    We can also use break and continue together to control
    the flow of a program. For example,*/

    let mut number = 0;

    loop {
        number += 1;

        // condition to skip the iteration
        if number == 3 {
            continue;
        }

        // condition to exit the loop
        if number > 5 {
            break;
        }

        println!("{}", number);
    }

    /*
    Here, the continue keyword,

    if number == 3 {
        continue;
    }

    skips the iteration when the value of the number variable is 3.

    Similarly, the break keyword,

    if number > 5 {
        break;
    }

    terminates the loop if the value of the number variable is greater than 5.
     */
}
