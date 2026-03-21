## Enums and Pattern Matching

Enums allow you to define a type by enumerating its possible variants.

Topics covered in this chapter. 

- Define and use of an enum
- Introduce a useful enum named `Option`
- Pattern matching with the `match` expression

### Defining an Enum

Enums give you a way of grouping variants of values. For example,
a set of variant shapes like `Rectangle`, `Circle`, and `Triangle`:

--- 
    enum Shapes {
        Circle,
        Rectangle,
        Triangle,
    }
--- 


### Enum values

We create an instance of the variants of Shape like this:

---
    let circle1 = Shapes::Circle;
    let rectangle1 = Shapes::Rectangle;
    let triangle1 = Shapes::Triangle;
---

The signature of a function that takes any variant of Shapes is as follows:

---
    fn draw_shape(a_shape : Shapes) {
    }
---

The invocation of the function with any variant: 

---
    draw_shape(Shapes::Circle);
    draw_shape(Shapes::Rectangle);
    draw_shape(Shapes::Triangle);
---

One can also put data into an enum variant:

---
    enum Shapes {
        Circle(String),
        Rectangle(String),
        Triangle(String),
    }

    let circle1 = Shapes::Circle(String::from("A circle"));
    let rectangle1 = Shapes::Rectangle(String::from("A rectangle"));
    let triangle1 = Shapes::Triangle(String::from("A triangle"));

---

**The data one can put in a enum variant should not be confused the data type of the
variant.** One can define a variant with any data type and any number of types.

---
    enum Message {
      Quit,
      Move {x:i32, y:i32},
      Write(String),
      ChangeColor(i32,i32,i32),
    }
---

One can also define methods on enums:

---
      impl Message {
        fn call (&self) {
          // Method body defined here.
        }
      }

      let message1 = Message::Write(String::from("hello"));
      m.call();
---


### The Option Enum and Its Advantages over Null Values

Rust has no concept of null!
Rust has no concept of null!
Rust has no concept of null!
Instead Rust has enum named `Option<T>`:
---
    enum Option<T> {
      None,
      Some(T),
    }
---

One has to convert `Option<T>` to `T` before you can perform `T` operations.

### The `match` Control Flow Construct

---
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents (coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin => 25,
        }
    }

---

### Matching with Option<T>

---
    let five = Option::<i32>::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::<i32>::None);

    fn plus_one( x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }
---

### Catch-All Patterns and the _ Placeholder

The pattern `_` is a catch all pattern. Note `match` is exhaustive.  The
`match` arms patterns must cover all possibilities.


### Concise Control Flow with `if let`

---
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!(),
        _ => count += 1,
    }
---

        or equivalently

---
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!",state);
    } else {
        count += 1;
    }
---

