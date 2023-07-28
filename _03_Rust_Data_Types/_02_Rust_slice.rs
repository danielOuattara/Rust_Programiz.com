/*
Rust slice
============*/

fn main() {
    /*
    A Rust slice is a data type used to access portions of data
    stored in collections like arrays, vectors and strings.

    Suppose we have an array:

    let numbers = [1, 2, 3, 4, 5];

    Now, if we want to extract the 2nd and 3rd elements of this
    array, we can slice the array like this:

    // let slice = &array[1..3];

    Here, let's look at the right-hand side of the expression,

    - &numbers - specifies a reference to the variable numbers,
                 not the actual value
    - [1..3]   - is a notation for slicing the array from start_index 1
                (inclusive), to end_index 3 (exclusive)
    */

    // an array of numbers
    let numbers_1 = [1, 2, 3, 4, 5];

    // create a slice of 2nd and 3rd element
    let slice_1 = &numbers_1[1..3];

    println!("array = {:?}", numbers_1); // [1,2,3,4,5]
    println!("slice_1 = {:?}", slice_1); // [2,3]
    println!("-------");

    /*

    NOTE: A slice is not the actual data like integers or floats but a
    reference/pointer to the data block. That's why we have used the '&'
    symbol before the variable name.


    Omitting Indexes of a Rust Slice
    =================================

    While slicing a data collection, Rust allows us to omit either
    the start index or the end index or both from its syntax:

    &variable[start_index..end_index];

    1. Omitting the Start Index of a Slice:
    ---------------------------------------*/

    let numbers_2 = [1, 2, 3, 4, 5];

    // omit the start index in writing only, not in value extracted
    let slice_2 = &numbers_2[..3];

    println!("array = {:?}", numbers_2);
    println!("slice_2 = {:?}", slice_2);
    println!("-------");

    /*

    2. Omitting the End Index of a Slice
    -------------------------------------*/

    let numbers_3 = [1, 2, 3, 4, 5];

    // omit the end index, not in value extracted
    let slice_3 = &numbers_3[2..];

    println!("array = {:?}", numbers_3); // array = [1, 2, 3, 4, 5]
    println!("slice_3 = {:?}", slice_3); // slice_3 = [3, 4, 5]
    println!("-------");

    /*

    3. Omitting both Start and End Index of a Slice
    ------------------------------------------------*/

    let numbers_4 = [1, 2, 3, 4, 5];

    // omit the start index and the end index
    // reference the whole array
    let slice_4 = &numbers_4[..];

    println!("array_4 = {:?}", numbers_4);
    println!("slice_4 = {:?}", slice_4);
    println!("-------");

    /*
    Here, &numbers[..] includes .. without the start and end index.
    This means the slice starts from index 0 and goes up to last index
    (inclusive).

    It is equivalent to &numbers[0..5] which will produce the same slice
    and will reference the whole array.


    Mutable Slice in Rust
    =======================

    We can create a mutable slice by using the '&mut' keyword.

    Once the slice is marked as mutable, we can change values inside
    the slice. Let's see an example, */

    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
    println!("-------");

    /*
    Here, we have created a mutable array colors. Then, we have created
    a mutable slice sliced_colors with &mut array[1..3].

    Now, we can change the content of the mutable slice,

    sliced_colors[1] = "purple"

    We change the value of original slice sliced_colors at the 1st index
    from "yellow" to "purple".


    Frequently Asked Questions
    ============================

    How to slice a string in Rust ?
    --------------------------------

    Just like an array, we can also slice a string in Rust.*/

    let string = String::from("Hello World!");

    // slicing a string
    let slice = &string[0..8];

    println!("string = {:?}", string);
    println!("slice = {}", slice);
    println!("-------");

    //-------------------------

    let mut string_2 = String::from("John Doe");
    println!("string_2 = {:?}", string_2);

    // slicing a string
    let slice_2 = &mut string_2[0..4];

    println!("slice_2 = {}", slice_2);

    // slice_2.chars().nth(1) = "a";
    // slice_2[2] = "n";
    // slice_2[3] = "a";

    for i in slice_2.chars() {
        println!("{i}");
    }

    println!("slice_2 = {}", slice_2);
    println!("-------");

    /*
    To learn more about strings in Rust, visit Rust String.


    How to slice a vector in Rust ?
    ---------------------------------

    Creating a slice from a vector is very similar to how we
    slice an array.*/

    let vector = vec!['A', 'E', 'I', 'O', 'U'];

    // slicing a vector
    let slice = &vector[1..4];

    println!("vector = {:?}", vector);
    println!("slice = {:?}", slice);

    println!("-------");

    let mut vector_2 = vec!['A', 'E', 'I', 'O', 'U'];
    println!("vector_2 = {:?}", vector_2);

    // slicing a vector_2
    let slice_vector_2 = &mut vector_2[1..4];

    println!("slice_vector_2 = {:?}", slice_vector_2);

    slice_vector_2[0] = 'T';
    slice_vector_2[1] = 'A';
    slice_vector_2[2] = 'X';
    println!("slice_vector_2 = {:?}", slice_vector_2);

    /* To learn more about vectors in Rust, visit Rust Vector. */
}
