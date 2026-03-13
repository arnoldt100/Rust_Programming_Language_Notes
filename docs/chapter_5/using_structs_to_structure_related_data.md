## Using Structs To Structure Related Data 

### Defining and Instantiating Structs

If a struct is defined to be mutable, then the 
entire instance is mutable, specifically, all fields are
mutable.

Below is a struct definition.

---
        struct Structure1 {
          <key1> : <type1>,
          <key2> : <type2>,
                .
                .
                .
        }

---

*Question: How do we define default values for struct key, value pairs?*

To get a specific value from a struct we use the `.` notation.

The listing below demonstrates using the field init shorthand to
initialize a struct.

---
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String)->User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    fn build_user_init_shorthand(email: String, username: String)->User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }


---

The listing below demonstrates creating a new struct instance that
includes some of the values from another instance.

---

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User{
        active: true,
        username: String::from("arnold_the_jedi"),
        email: String::from("arnoldt100@hotmail.com"),
        sign_in_count: 1,
    };

    let user2 = User{
      active: user1.active,
      username: user1.username,
      email: user1.email,
      sign_in_count: user1.sign_in_count,
    };

    // At this point `user1` is invalid because the `=` acts like an assignment; it
    // moves the data from instance `user1` to `user2`.

    // We can also use the struct update syntax.
    let user3 = User {
        email: String::from("arnoldt@ornl.gov"),
        ..user2, // This must come last!
    };

    // At this point `user2` is invalid because the `=` acts like an assignment; it
    // moves the data from instance `user2` to `user3`.

---


Rust supports tuple structs.


src/main.rs
---
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    fn main () {
        let black = Color(0,0,0);
        let origin = Point(0,0,0);

        // `black` and `color` are different types even though
        // their fields have the same values.
        println!("The black colors: {}, {}, {}",black.0,black.1,black.2);
    }
---

The tuple struct fields need not be of the same type. The indexing of tuple
structures must be literal numbers because it's indexed statically at
compilation time instead of at runtime. Once defined, a tuple struct arity is
fixed. The fields are accessed by numerical index starting from 0. 

In Rust, a comma is required to differentiate a single-element tuple or tuple
struct from a simple grouping expression enclosed in parentheses. This syntax
is essential for the compiler to correctly parse the code.

---
    .
    .
    .
    // Correct way to instantiate a single-element tuple struct
    let single_struct = MyInt(42); 

    // Correct way to create a single-element tuple (comma is mandatory here)
    let single_tuple = (42,); 
    .
    .
    .

---

One can also define structs that don't have any fields. These are called unit-like
structs because they behave similarly to  `()`, More on these structs later.


*Question: I'm confused about tuple struct fields and where the field data values are
stored - on the heap versus stack.*



