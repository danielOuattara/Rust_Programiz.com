/*
Rust Array
=========== */

fn main() {
    /*

    An array is a list of elements of the same type. For example,
    if we want to store the first five natural numbers, we can
    create an array instead of creating five different variables.

    In Rust, we use the square brackets [] to create an array.*/

    // array of natural numbers
    let _arr = [1, 2, 3, 4, 5];

    /*
    Here, we have created an array named arr that has five numbers.


    Creating an Array in Rust
    =========================

    In Rust, we can create an array in three different ways:

    1- Array with data type
    2- Array without data type
    3- Array with default values

    Let's understand each of these array creation methods in detail.


    Array with Data Type in Rust
    ============================= */

    // initialization of array with data type
    let numbers_1: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array of numbers_1 = {:?}", numbers_1);

    /*

    - numbers - name of the array
    - [i32; 5] - i32 is the predefined data type of array elements
      and 5 is the size of the array
    - [1, 2, 3, 4, 5] - elements inside the array


    Array without Data Type in Rust
    =================================*/

    // initialization of array without data type
    let numbers_2 = [1, 2, 3, 4, 5];

    println!("array of numbers_2 = {:?}", numbers_2);

    /*
    - numbers - name of the array
    - [1, 2, 3, 4, 5] - element inside the array

    You can see we have not defined the data type and size of the array.
    In this case, the Rust compiler automatically identifies the data
    type and size by looking at the array elements.


    Array with Default Values in Rust
    ================================== */

    // initialization of array with default values
    let numbers_3: [i32; 5] = [3; 5];

    println!("Array of numbers_3 = {:?}", numbers_3);

    /*
    - numbers - name of the array
    - [i32; 5] - represents the data type (i32), and size (5) of the array
    - [3; 5] - is a repeat expression, here the value 3 will fill the array
      5 times


    Note: We can also omit the data type and size while creating an array
    of default values. */

    // initialize array with default values
    let numbers_4 = [3; 5];

    println!("Array of numbers_4 = {:?}", numbers_4);

    /*

    Revision: Different Ways to Create Array in Rust
    -------------------------------------------------

    Let's look at a complete example of how we can create arrays
    in Rust. */

    // an array without data type
    let a = [5, 4, 3, 2, 1];

    // an array with data type and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // an array with default values
    let c = [3; 5];

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    /*
    Note: We use :? in the println! function to print an entire array.


    Access Elements of Rust Array
    ==============================

    Each element in an array is associated with a unique sequence of
    numbers. This number is known as the array index.

    Suppose we have an array of colors, */

    let _colors = ["red", "green", "blue"];

    /*
    Here's what the array indexes look like:

    In Rust, we can access individual array elements using their
    corresponding array indexes:

    - colors[0] - access the element at index 0 (first element)
    - colors[1] - access the element at index 1 (second element)
    - colors[2] - access the element at index 2 (third element)

    Note: The array index always starts at 0; hence the first element
    of the array is at position 0, not 1.*/

    let colors = ["red", "green", "blue"];

    // accessing element at index 0
    println!("1st Color: {}", colors[0]);

    // accessing element at index 1
    println!("2nd Color: {}", colors[1]);

    // accessing element at index 2
    println!("3rd Color: {}", colors[2]);

    /*

    Mutable Array in Rust
    =======================

    In Rust, an array is immutable, which means we cannot change its
    elements once it is created.

    However, we can create a mutable array by using the mut keyword
    before assigning it to a variable. */

    // create a mutable array in rust

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("original array = {:?}", numbers);

    // change the value of the 3rd element in the array
    numbers[2] = 0;

    println!("changed array = {:?}", numbers);

    /*

    Here, we have assigned a new value of 0 to the third element in
    the array.

    numbers[2] = 0;

    We changed the element at index 2 (third element) from 3 to 0.
    This is possible because we have created the numbers array as
    mutable.

    NOTE: Values inside an array can only be modified but cannot be
          deleted because the size of the array is fixed after
          initialization.


    Looping Through an Array in Rust
    =================================

    In Rust, we can use the 'for..in' loop to iterate through an
    array.*/

    let colors = ["red", "green", "blue"];

    // loop through an array to print its index and value
    for index in 0..3 {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }

    /*
        Here, the loop runs for 3 times ( 0 to 2 ). In each iteration of
        the loop, the value of index will be 0, 1, and 2.

        And we have used that index to access elements of the array.


    Frequently Asked Questions
    ===========================

    Can we create a dynamic array in Rust ?
    ----------------------------------------
    In a dynamic array, there is no definite size, and we can
    add as many elements as possible to the array.

    However, Rust doesn't allow us to create a dynamic array
    because the compiler needs to have a defined size to allocate
    space for the array.


    What are some of the features of arrays ?
    ------------------------------------------
    Some of the features of arrays in Rust are as follows:

    - An array can only include elements of the same data type.

    - We have to define the size of the array before using it,
      and we cannot change it after initialization.

    - Array elements are stored in a stack in sequential memory
      blocks.

    - Each element of the array is associated with a unique number
      called an array index, and we use array indexes to access
      the associated elements in the array.



    How can we find the length of an array in Rust ?
    -------------------------------------------------
    We can use the 'len()' method to find the length of a Rust
    array */

    let numbers_5 = [1, 2, 3, 4, 5];

    println!("Array Length: {}", numbers_5.len());

    // Output: Array Length: 5
}
