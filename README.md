# Rust_Programming_Language_Notes


## Chapter 3 Common Programming Concepts

### Variables and Mutability

All variables default to immutable. However,  you have the option are to make
your variables mutable.


#### Constants

Like immutable variables, constants are values that are bound to a name and are
not allowed to change, but there are a few differences between constants and
variables.

Here’s an example of a constant declaration:

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

Rust’s naming convention for constants is to use all uppercase with underscores
between words.

Constants are valid for the entire time a program runs, within the scope in
which they were declared.

#### Shadowing

Variables are declared by use of the let keyword:

    let x = 5;

You can declare a new variable with the same name as a previous variable.
Rustaceans say that the first variable is shadowed by the second, which means
that the second variable is what the compiler will see when you use the name of
the variable. In effect, the second variable overshadows the first, taking any
uses of the variable name to itself until either it itself is shadowed or the
scope ends. 

### Data Types

Rust is a statically typed language, which means that it must know the types of
all variables at compile time.

#### Scalar Types

Rust has 4 primary types: integers, floating-point numbers, Booleans, and characters.

##### Integer Types

| Length | Signed |  Signed Ranged  |  Unsigned   | Unsigned Range |   
| ------ | ------ | --------------- | ----------- | -------------- |
| 8-bit  | i8     | -128:127        |    u8       |  0:255         |
| 16-bit | i16    | -32,768:32,767  |    u16      |  0:65,535      |
| 32-bit | i32    |       ...       |    ...      |   ...          |
| 64-bit | i64    |      ...        |    ...      |   ...          |
| 128-bit | i128  |      ...        |    ...      |   ...          |
| arch   | isize  |                 |    usize    |                |

Each signed variant can store numbers from –2<sup>n-1</sup> to 2<sup>n-1</sup> – 1 inclusive,
where n is the number of bits that variant uses.

Unsigned variants can store numbers from 0 to 2<sup>n</sup> – 1.

##### Floating Point Types

* f32 - 32-bit
* f64 - 64-bit

##### Boolean Type

Two values: `true` or `false`


##### Character Type

Note that we specify char literals with single quotes.

#### Compound Types

##### Tuples 

Variable indexing on a heterogeneous type like a tuple makes it impossible for
the compiler to infer the type of the expression.


    fn main () {
        let x:(i32,f64,u8) = (500,6.4,1);
        let five_hundred = x.0;
        let six_point_four = x.1;

        let  (a,b,c) =  x;
        println!("The value of a is {a}.");
    }

##### Arrays

Every element of an array must have the same type. Unlike arrays in some other
languages, arrays in Rust have a fixed length.


    fn main() {
      let a = [1,2,3,4,5];
      let a: [i32;5] = [1,2,3,4,5];
      let a = [3;5];
    }


### Functions

Rust code uses snake case as the conventional style for function and variable
names, in which all letters are lowercase and underscores separate words.

#### Parameters

In function signatures, you must declare the type of each parameter. We
separate parameters by a commas.


#### Statements and expression

* Statements are instructions that perform some action and do not return a value.
Semicolons are generally used to end statements. Statements return the unit type.

* Expressions evaluate to a resultant value.

#### Functions with Return Values
Functions can return values to the code that calls them. We don’t name return
values, but we must declare their type after an arrow (->). In Rust, the return
value of the function is synonymous with the value of the final expression in
the block of the body of a function. You can return early from a function by
using the return keyword and specifying a value, but most functions return the
last expression implicitly.

`()` is the unit type.

#### The main program
- **Entry Point:** It is the single required starting point for binary crate
- **Signature:** The basic main function takes no parameters and returns nothing


### Control Flow

#### if Expressions

The basic form of an `if` expression is


    if condition1 {
        ... statements of expression ... 
    } else {
        ... statements of expression ... 
    }

`condition1` must evaluate to a `bool`
The `else` expression is optional.

Because `if` is an expression we can assign it to the right side of a `let`
statement. Remember that blocks of code evaluate to the last expression in 
them.

#### Repetition Code with `loop`

The loop keyword tells Rust to execute a block of code over and over again. You 
must explicitly tell it to stop the loop execution with `CTRL-C`.
 
    fn main() {
        loop {
          println!("again");
        }
    }

#### Returning Values from Loops

To return a value from `loop`, you can add value after a break expression.

    fn main() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {number}");

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter*2;
            }
        };
        println!("The result is: {result}");
    }

#### Loop Labels to Disambiguate Between Multiple Loops

If you have multiple loops `break` and `continue` apply to the 
innermost loop at that point. One can optionally specify a loop
label on a loop that you can then use with the break or continue
to indicate that those keywords apply to the labeled loop.

    fn main() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            'counting_down: loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }

                if count == 2 {
                   break 'counting_up;
                }

                remaining += -1;
            }
            count += 1;
    }

#### Conditional Loops with `while`

Used to run a code while a condition evaluates to `true`. `while` loops 
can have labels that `break` and `continue` can be used with.

    fn main() {
        let mut number = 3;
        while number != 0 {
            println!("{number}");
            number -= 1;
        }
        println!("LIFTOFF");
    }


#### Looping Through a Collection with `for` 

For loops can have labels that `break` and `continue` can be used with.

    fn main () {
        let a = [10,20,30,40,50];
        for element in a {
            println!("The element value is {element}");
        }
    }

## Chapter 4 Understanding Ownership
[Chapter 4 Understanding Ownership](./docs/chapter_4/understanding_ownership.md)


