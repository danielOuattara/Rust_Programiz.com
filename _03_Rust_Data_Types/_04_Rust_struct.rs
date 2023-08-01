/*
Rust struct
================ */

fn main() {
    /*
    Rust structs or structures are user-defined data types
    used to store different types of data together.

    Suppose we want to store a person's name, age, and height.
    To do this, we can create variables for each property/field.
    */

    let _person_name: String = String::from("John Doe");
    let _person_age: u8 = 18;
    let _person_height: u8 = 178;

    /*
    The problem with this approach is we have to maintain all
    these variables separately. To store these fields for more
    than one person, we will have to create different variables
    for each person.

    Instead, we can create a struct to store all the fields
    together as a single unit


    Defining a Struct in Rust
    ==========================

    In Rust, we use the 'struct' keyword to define a structure.
    The syntax of a structure is:

    struct StructName {
        field1: data_type,
        field2: data_type,
        field3: data_type
    }

    Here,

    - struct - keyword to define a structure
    - StructName - name of the structure
    - field#: data_type - name and data type of the fields inside
      the struct.

    Let's look at an example. */

    struct Person1 {
        name: String,
        age: u8,
        height: u8,
    }

    /*
    Here, we have defined a structure named Person1. It contains
    three fields:

    - name   - with data type String
    - age    - with data type u8
    - height - with data type u8


    Instantiating Rust Structs
    ==========================

    To use a structure in Rust, we first have to create an instance
    from structures.*/

    // define a structure
    struct Person2 {
        name: String,
        age: u8,
        height: u8,
    }

    // create an instance
    let _person2 = Person2 {
        name: String::from("John Doe"),
        age: 18,
        height: 178,
    };

    /*
    Here, we have initialized the values of the name, age and height
    fields of the Person struct. This process of initializing the
    values of struct fields is known as an instantiation of a struct.

    Note: The struct definition is a template, and the struct instances
          fill in that template with data.


    Accessing Fields of a Struct
    =============================

    We can use a struct instance along with the dot . notation to
    access values of fields in that structure. */

    // define a Person struct
    struct Person3 {
        name: String,
        age: u8,
        height: u8,
    }

    // instantiate Person struct
    let person3 = Person3 {
        name: String::from("John Doe"),
        age: 18,
        height: 178,
    };

    // access value of name field in Person struct
    println!("Person name = {}", person3.name);

    // access value of age field in Person struct
    println!("Person age = {}", person3.age);

    // access value of height field in Person struct
    println!("Person height = {}", person3.height);
    println!("-------");

    /*
    Here,

    - person.name   - reads the field name of the Person struct (John Doe)
    - person.age    - reads the field age (18)
    - person.height - reads the field height (178)


    Destructuring Fields of a Rust Struct
    ======================================

    Destructuring is the process of breaking down fields of a data
    type (array, tuple, struct, etc.) into smaller variables.

    We can break down the struct fields into smaller variables in Rust.

    Suppose we have a struct and a struct instance*/

    struct Person4 {
        name: String,
        age: u8,
        height: u8,
    }

    let person4 = Person4 {
        name: String::from("John Doe"),
        age: 18,
        height: 178,
    };

    // We can now perform destructuring using:

    // destructuring the Person struct
    let Person4 { name, age, height } = person4;

    /*
    Now, we access the 'name', 'age' and 'height' fields using the
    field names directly:

    - name instead of person.name
    - age instead of person.age
    - height instead of person.height

    However, you should note that the name of the variables while
    destructuring should be the same as the name of the fields.
    */

    // Example: Destructuring Fields of Struct

    // define a Person struct
    struct Person5 {
        name: String,
        age: u8,
        height: u8,
    }

    // instantiate Person struct
    let person5 = Person5 {
        name: String::from("Jade Doe"),
        age: 18,
        height: 178,
    };

    // destructure Person struct into name, age and height variables
    let Person5 { name, age, height } = person5;

    println!("Person name = {}", name);
    println!("Person age = {}", age);
    println!("Person height = {}", height);
    println!("-------");

    /*
    Here, the destructing happens with this expression,

    let Person5 { name, age, height } = person5;

    The pattern on the left has declarations, and the right side of
    the expression has a struct instance.

    - On the left side of the expression, we are making let
      declarations for the Person struct with field name, age and
      height.
    - On the right side of the expression, we assign the instantiated
      struct of the Person.

    As a result, we get the name, age and height of the person and print
    it to the screen.

    Frequently Asked Questions
    ===========================

    How to create a mutable struct in Rust ?
    ----------------------------------------
    To create a mutable struct, we use the 'mut' keyword while declaring
    the structure variable. */

    // define a Point struct
    struct Point {
        x: i32,
        y: i32,
    }

    // instantiate Point struct to a mutable structure variable
    let mut point = Point { x: 0, y: 0 };

    println!("Before change:");
    println!("Point x = {}", point.x); // x = 0
    println!("Point y = {}", point.y); // y = 0

    // change the value of x field in mutable point struct
    point.x = 5;

    println!();
    println!("After change:");
    println!("Point x = {}", point.x); // x = 5
    println!("Point y = {}", point.y); // y = 0
    println!("-------");

    /*
    What are tuple structs ?
    -------------------------
    Tuple structs are a hybrid between a tuple and a struct.
    All tuple structs have a name, but their fields don't */

    // a tuple struct
    #[derive(Debug)]

    struct Point2(i32, i32);

    // instantiating a tuple struct with values
    let point2 = Point2(1, 2);

    println!("{:?}", point2);
    println!("-------"); // Point(1, 2)

    /*
    Output

    Point(1, 2)

    Here, struct Point(i32, i32) is a tuple struct with two fields
    of type i32.

    Note: Add #[derive(Debug)] above the struct definition to allow
    Rust to print the entire structure.


        */
}
