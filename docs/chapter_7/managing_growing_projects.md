## Managing Growing Projects with Packages, Crates and Modules

Rust has a number of features that allow one to manage growing 
projects:

- **Packages** 
- **Crates**
- **Modules and use**
- **Paths**

This chapter will discuss encapsulating implementation details,
managing scope of variables, functions, etc.

### Packages and Crates

A *crate* is the smallest amount of code that the Rust compiler considers at a
time. A `crate` can have one of two forms: a binary crate or a library crate. A
binary crate has a main function. A library crate doesn't have a main function.

A `crate root` is a source file that the Rust compiler starts from and makes
up the root module of your crate. More on crates later.

A `package` is a bundle of one or more crates and its contains a `Cargo.toml`
file. A package can contain multiple binary crates and optionally one library
crate.  One adds additional binary crates by placing files in the src/bin
directory: Each file will be a separate binary crate.

### Defining Modules to Control Scope and Privacy 

An example of nested modules that contain functions:

*src/lib.rs*
---

    mod A_level_1_mod {
        mode A_level_2_mod {
          fn foo() {}
          fn bar() {}
        }
    }

    mod B_level_1_mod {
        fn goo() {}
        fn hoo() {}
    }

---

### Paths for referring to an Item in the Module Tree

A path can take two forms:

- An absolute path
- A relative path

---
    mod A_level_1_mod {
        mode A_level_2_mod {
          fn foo() {}
          fn bar() {}
        }
    }

    mod B_level_1_mod {
        fn goo() {}
        fn hoo() {}
    }

    // Absolute path
    crate::A_level_1_mod::A_level_2_mod::foo();

    // Relative path 
    A_level_1_mod::A_level_2_mod::foo();
---

The above listing will fail!! We have the correct paths but recall that
all items ( functions,methods, structs, enums, modules, constants, etc.) are
private to the parent module by default. We expose the child module items
by the keyword `pub`.


---
    mod A_level_1_mod {
        pub mode A_level_2_mod {
          pub fn foo() {}
          fn bar() {}
        }
    }

    mod B_level_1_mod {
        fn goo() {}
        fn hoo() {}
    }

    // Absolute path
    crate::A_level_1_mod::A_level_2_mod::foo();

    // Relative path 
    A_level_1_mod::A_level_2_mod::foo();
---

Note the we needed to use `pub` on all items in the path to and including `foo`.


### Best Practices for Packages with a Binary and Library

A package can contain both a `src/main.rs` crate root as well as 
a `src/lib.rs` and both crates will have the package name by default.

The module tree should defined in `src/lib.rs` Then any public items can
be used in the binary crate by starting with the name of the package.


### Starting Relative Paths with Super

Using `super` allows us to reference an item that we know is the parent
module.

### Making Structs and Enums public

Emum: Use `pub` to designate an enum as public - by default, all variants will
be be designated as public.

Structs: Use `pub` to designate a struct as public - by default all fields in a struct are 
private, One must explicitly designate the struct as public and the fields as public
for visibility of the sruct fields.


### Bringing Paths into Scope with the `use` Keyword

Writing out the paths to call items in Rust can be inconvenient. Adding `use` and a path scope
can reduce the burden of writing out paths to ites=ms.

---
    mod A_level_1_mod {
        pub mode A_level_2_mod {
          pub fn foo() {}
          fn bar() {}
        }
    }

    pub struct Appetizer {
        pub toast: String, 
    }

    mod B_level_1_mod {
        fn goo() {}
        fn hoo() {}
    }

    // Absolute path
    crate::A_level_1_mod::A_level_2_mod::foo();

    // Relative path 
    A_level_1_mod::A_level_2_mod::foo();

    use crate::A_level_1_mod::A_level_2_mod;
    foo();

    use crate::A_level_1_mod::Appetizer;
    let appetizer1 = Appetizer{toast:String::from("wheat")};
---

### Creating Idiomatic use Paths 

For structs, enums, and other items with use specify the full path.
For fuctions, bring the parent module into scope.

### Providing New Names with the `as` Keyword

(Resume reading here)

