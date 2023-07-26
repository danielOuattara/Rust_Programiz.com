/*
Rust Type Casting
================== */

pub fn main() {
    /*
    Type casting allows us to convert variables of one data type
    to another. In Rust, we use the 'as' keyword to perform
    type casting.
    */

    // create a floating-point variable
    let decimal: f64 = 54.321;
    println!("decimal = {decimal}");

    // convert floating point type to integer type
    let integer = decimal as u16;
    println!("integer = {integer}");

    /*
    Here, 'decimal as u16' expression converts f64 floating-point
    type to u16 integer type.
    */

    // assign a floating point f64 value to 'decimal' variable
    let decimal: f32 = 64.31;

    // convert 'decimal' variable to u16 integer type
    let integer = decimal as u16;

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);

    /*
    Here, the variable 'decimal' with floating point value 64.31
    is converted to an integer value 64 of type u16 with the help
    of 'as' Rust keyword.

    We are converting data from one type to another type manually
    using the 'as' keyword. This way of type casting is also known
    as Explicit Type Casting.


    Type Conversion: Character to Integer in Rust
    ===============================================*/

    let character: char = 'A';

    //convert 'char' type to 'u8' integer type
    let integer = character as u8;

    println!("character = {}", character);
    println!("integer = {}", integer);

    println!("Number of byte(s) for u8: {}", std::mem::size_of::<u8>()); // 4 bytes
    println!("Number of byte(s) for f32: {}", std::mem::size_of::<f32>()); // 4 bytes
    println!("Number of byte(s) for f64: {}", std::mem::size_of::<f64>()); // 4 bytes

    /*
    In Rust, the 'char' data type is internally stored as
    a Unicode Scalar Value. Unicode Scalar Value is simply
    the numeric representation of a character in Unicode
    standard also known as a code point.

    The Unicode value of character A is 65. So, we get the
    output of 65 when converting the character A to an integer.


    Type Conversion: Integer to Character in Rust
    ==============================================

    We can also convert integer type to a character type. */

    // only u8 integer data type can be converted into char
    let integer: u8 = 65;

    // convert integer to char using the as keyword
    let character = integer as char;

    println!("integer = {}", integer);
    println!("character = {}", character);

    /*
     In the example above, the integer value 65 is the Unicode code
     for character A. Thus after type casting, we get character A
     as the output. Every character has an Unicode code associated
     with it.


    Error while Converting Integer to Character
    =============================================

     We are only allowed to use u8 integers while performing type
     casting between integer and character. If we use any other
     integer type and convert it to a character, we will get an
     error. */

    // let integer: i32 = 65;

    // // convert integer to char using the as keyword
    // let character = integer as char;

    // println!("integer = {}", integer);
    // println!("character = {}", character);

    /*

    -----------------------------------------
    error[E0604]: only `u8` can be cast as `char`, not `i32`
       --> _07_Rust_type_casting.rs:101:21
        |
    101 |     let character = integer as char;
        |                     ^^^^^^^^^^^^^^^ invalid cast
        |
    help: try `char::from_u32` instead (via a `u32`)
       --> _07_Rust_type_casting.rs:101:21
        |
    101 |     let character = integer as char;
        |                     ^^^^^^^^^^^^^^^

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0604`.

    -----------------------------------------


    Here, we have used i32 data type instead of u8.
    Hence we get an error.

    It's because Unicode Scalar Values are small integer numbers
    and fit in the range of u8 data type.

    Type Casting: Boolean to Integer in Rust
    ========================================= */

    let boolean1: bool = false;
    let boolean2: bool = true;
    println!("boolean1 = {}", boolean1); // false
    println!("boolean1 = {}", boolean2); // true

    // convert boolean type to integer
    let integer1 = boolean1 as i32;
    let integer2 = boolean2 as i32;
    println!("integer1 = {}", integer1); // 0
    println!("integer2 = {}", integer2); // 1

    /*
        Here, boolean data type 'false' and 'true' are converted to
        integer 0 and 1 respectively.


    Limitations of Type Casting
    ===============================

    There are limitations while performing type casting in Rust.
    Not all data types are converted to one another.

    For example, we cannot convert a floating type to a character.*/

    // let decimal: f32 = 65.321;

    // // convert float to char data type
    // let character = decimal as char;

    // println!("decimal = {}", decimal);
    // println!("character = {}", character);

    /*
    -------------------------------------------------
    error[E0604]: only `u8` can be cast as `char`, not `f32`
       --> _07_Rust_type_casting.rs:163:21
        |
    163 |     let character = decimal as char;
        |                     ^^^^^^^^^^^^^^^ invalid cast
        |
    help: try `char::from_u32` instead (via a `u32`)
       --> _07_Rust_type_casting.rs:163:21
        |
    163 |     let character = decimal as char;
        |                     ^^^^^^^^^^^^^^^

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0604`.
    -------------------------------------------------

    Here, we have tried to convert the float type to char, hence
    we get an error. The error says that Rust is expecting a u8
    data type for conversion not f32.

    Frequently Asked Questions
    ===========================

    How to convert a floating type to a character ?
    ------------------------------------------------

    To convert a floating type to a character, you have to first
    convert floating value f32 to u8 integer type and then convert
    it to char. */

    let decimal: f32 = 65.321;

    // convert float to integer data type
    let integer = decimal as u8;

    // convert integer to char data type
    let character = integer as char;

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);
    println!("character = {}", character);

    /*

    How to perform Implicit Type Casting in Rust?
    -----------------------------------------------

    In implicit type casting, the compiler automatically converts
    one data type to another. This is also known as Automatic Type
    Casting.

    However, Rust programming doesn't support Implicit Type Casting
    between primitive/scalar types. For example, */

    // let integer: u8 = 32.8;
    // println!("Integer = {}", integer);

    /*
    -----------------------------------------------------------------
    error[E0308]: mismatched types
       --> _07_Rust_type_casting.rs:226:23
        |
    226 |     let integer: u8 = 32.8;
        |                  --   ^^^^ expected `u8`, found floating-point number
        |                  |
        |                  expected due to this

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0308`.
    -------------------------------------------------------------------


    Here, we have assigned a floating-point value to an integer type
    variable. Since Rust doesn't support implicit type casting, the
    value 32.8 is not automatically converted to integer value.
    Hence, we get an error.


    How to perform Explicit Type Casting in Rust?
    -----------------------------------------------

    As explained in the above article, we use the 'as' keyword in Rust
    to perform explicit type casting.*/

    let pi: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066;

    let integer = pi as f32;

    println!("pi(f64) = {}", pi);
    println!("pi(f32) = {}", integer);

    //---

    let pi: f64 = 3.14159265359;

    let integer = pi as u32;

    println!("pi = {}", pi);
    println!("pi(u32) = {}", integer);
}
