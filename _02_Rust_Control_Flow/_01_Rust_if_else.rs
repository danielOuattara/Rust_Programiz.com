/* Rust If else
=============== */

pub fn main() {
    /*

    In computer programming, we use the if and if..else statements
    or expressions to run a block of code when a specific condition
    is met.

    For example, a student can be assigned grades to a subject
    based on their overall score.

    - if the score is above 90, assign grade A
    - if the score is above 75, assign grade B
    - if the score is above 65, assign grade C


    Boolean Expression
    ===================

    Before we learn about if..else expressions, let's quickly
    understand boolean expressions.

    A boolean expression is an expression (produces a value) which
    returns either true or false (boolean) as the output: */

    let x = 7;

    // example of boolean expression
    let condition = x > 5;

    println!("condition is {}", condition); // condition is true

    /*
    Here, 'x > 5' is a boolean expression that checks whether the
    value of variable x is greater than 5. As 7, the value of
    variable x is greater than 5, the condition variable is assigned
    to true. Hence, condition is true is seen as output.


    Rust if Expression
    ====================

    An if expression executes the code block only if the condition is
    true. The syntax of the if expression in Rust is:

    if condition {
        // code block to execute
    }

    If the condition evaluates to

    - true - the code inside the if block is executed
    - false - the code inside of the if block is not executed


    Example: if expression
    -----------------------*/

    let number = 10;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    }

    println!("End of program");

    /*

    In the example above, we created a variable number and checked
    whether its value is greater than 0. Notice the condition,

    number > 0

    Since the variable number is greater than 0, the condition
    evaluates to true. As a result, we see the code block inside
    of curly braces being executed.

    Suppose we change the number variable to a negative integer. Let's say -2.

    let number = -2;

    The output will be: End of program

    This is because the condition -2 > 0 evaluates to false and the
    body of if is skipped.


    Rust if..else Expressions
    ==========================

    The if expression can occasionally also include an optional
    else expression. The else expression is executed if the
    condition in if is false.

    The syntax for the if..else expression in Rust is:

    if condition {
        // executes when condition is true
    } else {
        // executes when condition is false
    }

    1. If condition evaluates to true,

        the code block inside if is executed
        the code block inside else is skipped

    2. If condition evaluates to false,

        the code block inside the if block is skipped
        the code block inside the else block is executed


    Example: if..else expression
    ----------------------------- */

    let number = -2;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    } else {
        println!("{} is less than or equal to 0", number);
    }

    /*
    Here, the variable number has a value of -2, so the
    condition number > 0 evaluates to false. Hence, the
    code block inside of the else statement is executed.

    If we change the variable to a positive number, let's
    say 10.

    // let number = 10;

    The output of the program will be:  10 is greater than 0

    Here, the condition number > 0 evaluates to true. Hence,
    the code block inside of the if statement is executed.


    Rust if..else if Expressions
    =============================

    We can evaluate multiple conditions by combining if and
    else in an else if expression. An if..else if expression
    is particularly helpful if you need to make more than two
    choices. The syntax for if with else if expression looks
    like this:

    if condition1 {
        // code block 1
    } else if condition2 {
        // code block 2
    } else {
        // code block 3
    }

    Here,

    1. If condition1 evaluates to true,

        code block 1 is executed
        code block 2 and code block 3 is skipped

    2. If condition2 evaluates to true,

        code block 2 is executed
        code block 1 and code block 3 is skipped

    3. If both condition1 and condition2 evaluate to false,

        code block 3 is executed
        code block 1 and code block 2 is skipped

    Example: if..else if..else Conditional
    --------------------------------------- */

    let number = -2;

    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is equal to 0", number);
    }

    /*
    In this example, we check whether a number is positive,
    negative or zero. Because number = -2 is less than 0
    which satisfies the condition: number < 0, the else if
    block is executed.


    Nested if..else
    ================

    You can use if..else expressions inside the body of other
    if..else expressions. It is known as nested if..else in Rust.

    if outer_condition {
        // outer code block

        if inner_condition {
            // inner code block 1
        } else {
            // inner code block 2
        }
    }

    // outside if code block */

    let number = -2;

    if number < 0 {
        // if outer condition evaluates to true evaluate the inner condition
        if number == -2 {
            println!("The current number is -2");
        } else {
            println!("The current number is {}", number);
        }
    }

    /* The current number is -2

    In this example,

    1. The outer_condition number < 0 evaluates to true as the
       number variable is assigned to a value of -2. Thus, the
       outer code block is evaluated.

    2 .The outer code block contains an inner_condition to check
       number == -2, which is again true. Thus, the inner code
       block of the inner if expression is executed.

    3 .The inner code block prints "The current number is -2" to
       the screen.

    4 .The inner else block is skipped because the inner_condition
       evaluated to true.

     */
}
