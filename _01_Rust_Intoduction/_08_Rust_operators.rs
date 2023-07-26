/*  Rust Operators
=================== */

pub fn main() {
    /*

    An operator is a symbol that performs operations on values
    or variables. For example, '-' is an operator that performs
    subtraction between two values.

    Rust programming provides various operators that can be
    categorized into the following major categories:

    - Arithmetic Operators
    - Compound Assignment Operators
    - Logical Operators
    - Comparison Operators

    Arithmetic Operators in Rust
    =============================

    We use arithmetic operators to perform addition, subtraction,
    multiplication, and division.

    Here's a list of various arithmetic operators available in Rust.
    We have used the variable names 'a' and 'b' in the example.

    ------------------------------------------------
    Operator                     Example
    ------------------------------------------------
    + (Addition)                 a + b
    ------------------------------------------------
    - (Subtraction)              a - b
    ------------------------------------------------
    * (Multiplication)           a * b
    ------------------------------------------------
    / (Division)                 a / b
    ------------------------------------------------
    % (Remainder)                a % b
    ------------------------------------------------
    */
    let a = 20;
    let b = 2;

    // add two variables using '+' operator
    let x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using '-' operator
    let y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using '*' operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    //-------------------------------

    let dividend = 21;
    let divisor = 8;

    // arithmetic division using / operator with integers
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    /* In the above example, we use the '/' operator to divide
    two integers 21 and 8. The output of the operation is 2.

    In standard calculation, '21 / 8' gives 2.625. However,
    in Rust, when the '/' operator is used with integer values,
    we get the quotient (integer) as the output.


    If we want the actual result, we should use the '/' operator
    with floating-point values.
    */

    let dividend = 21.0;
    let divisor = 8.0;

    // arithmetic division using / operator with floating point values
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    /* Here, both dividend and divisor variables are assigned
    floating point values. Thus, the division operation returns
    a floating point result of 2.625. */

    // Remainder Operator
    // ------------------

    let dividend = 21;
    let divisor = 8;

    // arithmetic remainder using % operator
    let remainder = dividend % divisor;

    println!("{} % {} = {}", dividend, divisor, remainder);

    /* Here, we use the remainder operator '%' with two integers:
     21 and 8. The output of the operation is 5.

     The remainder operator '%', as the name suggests, always
     returns the remainder after division.


     Assignment Operator
    ====================

     We use an assignment operator to assign a value to a variable.
      */
    let _x = 5;

    /* Here, the '=' operator assigns the value on the right to
    the variable on the left.


    Compound Assignment Operators
    ==============================

    We can also use an assignment operator and an arithmetic
    operator, known as a compound assignment operator. */
    let mut x = 1;

    // compound assignment operators
    x += 3;
    println!("x = {x}"); // 4

    /* Here, '+=' is a compound assignment operator known as an
    addition assignment. It first adds 3 to the value of x (1)
    and assigns the final result (4) to x.

    Here's a list of various compound assignment operators in Rust.


    -------------------------------------------------------------------------
    Operator                                Example             Equivalent
    -------------------------------------------------------------------------
    += (Addition  assignment)                 a += b            a = a + b
    -------------------------------------------------------------------------
    -= (Subtraction  assignment)              a -= b            a = a - b
    -------------------------------------------------------------------------
    *= (Multiplication  assignment)           a *= b            a = a * b
    -------------------------------------------------------------------------
    /= (Division  assignment)                 a /= b            a = a / b
    -------------------------------------------------------------------------
    %= (Remainder  assignment)                a %= b            a = a % b
    -------------------------------------------------------------------------
    */

    let mut a = 2;

    // arithmetic addition and assignment
    a += 3;

    println!("a = {}", a); // 5

    /*
    Comparison Operators
    =====================

    We use comparison operators to compare two values or variables.
    For example: 6 > 5

    Here, the relational operator '>' (greater than) checks if 6 is
    greater than 5.

    A relational operator returns:

    - 'true' if the relation between two values is correct
    - 'false' if the relation is incorrect

    Note: Comparison operators are also known as 'relational operators'.

    Here's a list of comparison operators available in Rust.

    ----------------------------------------------
    Operator                            Example
    ----------------------------------------------
    > (Greater than)                    a > b
    ----------------------------------------------
    < (Less than)                       a < b
    ----------------------------------------------
    >= (Greater than or equal to)       a >= b
    ----------------------------------------------
    <= (Less than or equal to)          a <= b
    ----------------------------------------------
    == (Equal to)                       a == b
    ----------------------------------------------
    != (Not Equal to)                   a != b
    ----------------------------------------------
    */

    let a = 7;
    let b = 3;

    // use of comparison operators
    let c = a > b;
    let d = a < b;
    let e = a == b;

    println!("{} >= {} is {}", a, b, c);
    println!("{} <= {} is {}", a, b, d);
    println!("{} == {} is {}", a, b, e);

    /*
    Logical Operators
    ======================

    We use logical operators to perform logical decisions
    or operations. A logical operation returns either true
    or false depending on the conditions.

    // (5 < 6) && (7 > 4)

    Here, '&&' is the logical 'AND' operator that returns true
    if both conditions are true. In our example, both conditions
    are true. Hence the expression is true.

    There are mainly 3 logical operators in Rust:

    ----------------------------------------------
    Operator            Example
    ----------------------------------------------
    && (Logical AND)    expression_1 && expression_2
    ----------------------------------------------
    || (Logical OR)     expression_1 || expression_2
    ----------------------------------------------
        ! (Logical NOT)    !expression
    */

    let a = true;
    let b = false;

    // logical AND operation
    let c = a && b;

    // logical OR operation
    let d = a || b;

    // logical NOT operation
    let e = !a;

    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);

    /*

    Note: The logical 'AND' and 'OR' operators are also
    called 'short-circuiting logical operators' because
    these operators don't evaluate the whole expression
    in cases they don't need to. For example, in this
    expression

    false || true || false

    The || operator evaluates to true because once the
    compiler sees a single true expression, it skips
    the evaluation and returns true directly.
    */
}
