# Rust

## *Useful links*

- [rust's guidelines](https://rust-lang.github.io/api-guidelines/)
- [formatting](https://doc.rust-lang.org/std/fmt/#formatting-traits)
- [logger](https://docs.rs/log/latest/log/index.html)
- [iterator's api](https://doc.rust-lang.org/std/iter/trait.Iterator.html#)

## *miscellaneous*

variables are immutable by default.\
constant and function signatures must use type annotation. \
shadowing a variable may be a useful way to escape using the `mut` keyword

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```
## *expressions*

a code that evaluates to a value. Examples for expression are macros functions if loops (`x {}`).

```rust
let number = if condition { 4 } else { 6 };
```

## loops

labeled break/continue -> the specified label will be break/continue

when iterating over a range of values always use `for element in collection`, more safe.

## *rustup*

rustup installs rust's compilers and enabling channel switching (stable, beta, nightly).



## *unsafe keyword*

helps to bypass the compiler protection.

## *rust ownership*

heap allocated variables implement a trait with `drop` function, which frees the variable.

you can ***move*** variables, meaning the rust complier invalidates the previous owner.

### *borrowing*

taking a reference from variable, meaning not taking ownership.
A reference lifetime is from its deceleration till the last time that reference is **used**.\
Moreover you **cannot** have more than one mutable reference at a time but you can have **infinite** immutable references.

## *associated functions*

```rust

#[derive(Debug)]
struct A {
    a: u32,
    b: u32,
}

impl A {
    fn a_method(&self) -> u32 {
        self.a
    }
    fn solve(a: u32) -> f64 {
        a / 3.2
    }
}

fn main() {
    let object A {
        a: 4,
        b: 3,
    }

    printnl!("{object.a_method()}")
    printnl!("{object::solve(3)}")

}
```

## *enumeration*

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Test(A),
}
```

in rust there is no Null value. You can have a null representation by using the `Option<T>` enum.

```rust
enum Option<T> {
    None,
    Some(T)
}
```

To initialize a variable that can be null we use

```rust
fn main() {
    let a = Some(4);

    let res = match a {
        None => {
            println!("a is null");
            -1
        }
        // you can specify a parameter i to pass
        Some(i) => Some(i + 1),
    }
}
```

### *match statements*

```rust
let value = Some(3u8);
match value {
    Some(val) => println!("The value is {val}"),
    _ => (),
}
```

For defining a one arm instead of using `match` we can use `if let`:

```rust
let value = Some(3u8);

if let Some(val) = value {
    println!("The value is {val}");
} else {
    // like using _ => ()
    println!("the other cases");
}
```

## *managing projects in rust*


### **compiler flow**

1. look at the ***crate root***, `src/lib.rs` or `scr/main.rs`
2. inside the crate root modules will be referenced using `mod x` (inline, `src/x.rs`, `src/x/mod.rs` (the older version)), you can continue to create sub modules the same way.
3. 

### **Packages**

A Cargo feature that lets you build, test, and share crates.
Inside your package you can have one library.

### **Crates**

A tree of modules that produces a library (no main and not compiled) or executable.

### **Modules** and **use**

Let you control the organization, scope, and privacy of paths. Every sibling modules have public privileges to each other.

#### **rust `mod`**

a private/public collection of items: functions, traits, impl blocks, etc… modules are private by default use the `pub` keyword to make them public

```rust
use crate_y::x::y::z;
use crate_x::dodo as bobo;
// make the use public.
pub use x::y::z;
use std::io::{self, Write}; // std::io, std::io::Write

fn main() {
    let b: z = z::new();
}
```

### **Paths**

A way of naming an item. The path can be relative (`super::`).

## *error handling*

basic:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl Result {
    // returns result or panic the error.
    fn expect(&str error_message) {};
    fn unwrap() {};
}
```

To propagate an error instead of using `return Err(_)` we use `?` like this:

```rust
/*
if the read_to_string returns an error
it will `return` this error immediately. otherwise it will return the value.
*/
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

## *debug*

```rust
#[derive(Debug)]
struct S {
    a: u32,
    b: u32,
    c: char,
}



fn test() -> S {
    
    S {
        a: 3u32,
        b: 4u32,
        c: 'c',
    }
}
fn main() {
    println!("{:?}", test());
}
```

## *traits*

like interfaces but you can add default functions / methods that uses unimplemented functions / methods.

```rust
trait FooBar {
    fn foo(&self) -> String;

    fn bar(&self) -> String {
        foo() + ":)"
    }
}

impl FooBar for StructX {
    fn foo(&self) -> String {
        "foo foo bar"
    }
}
```

### *trait bound*

trait acts like data types where you can use them like types.

```rust
pub fn notify(item: &(impl Summary + Display))
// better
pub fn notify<T: Summary + Display>(item: &T)

// if too complex then you can use the where keyword.
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}

fn foobar() -> impl Display + Clone
```

Moreover we can implement every type that have specific traits this is called *blanket implementations*

```rust
impl<T: Display + PartialOrd> ToString for T {
    ...
}
```

## *lifetimes*

rust lifetimes are a tool used by the *borrow checkers* to check for valid code.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

the function `longest` takes two string slices and return a the string slice with a lifetime of the smallest slice.

The compiler can infer lifetimes when

1. the function / impl block have only one ref
2. it a method (contains &self).
3. more stuff may be added in the future.

## *closures*

### Closures' Traits

1. `FnOnce` - can be called once (moves captured values).
2. `FnMut` - mutate captured values and cannot move out captured values
3. `Fn` - doesn't mutated or move(out of closure) captured values / doesn't capture any values.

fn - function pointer

```rust
fn foo(bar: fn(i32) -> i32);
```

## *iterators*

are used to iterate over a data structure that implements the `Iterator` trait.

```rust
pub trait Iterator {
    type Item;
    // next consumes the iterator
    // every function using next is a consuming adaptor.
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

fn main() {
    let v = vec![1, 2, 3];
    for val in v.iter() {
        ...
    }
}
```

Note: *iterator adaptors* doesn't get called if not consumed by a *consuming adaptor*. 

### *iterators' useful functions*

1. `collect` - transform an iterator to a collection.
2. `map` - change each element using a closure.
3. `filter` - filter a iterator's elements using a closure.
4. `zip` - zip up two iterators into a single iterator of pairs.

## *macros*

to create a macro you can use `macro_rules!` or define new crate with function that gets a `TokenStream` and returns the modified one.

```rust
#[macro_export]
macro_rules! vec {
    /* 
     $x is an expression 
     that is separated by ,
     and can can have infinite values (*)
     then inside the arm repeat the push operation * times.
    */
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```


### macros types

- Derive -  `#[derive(Debug + Copy + Display)]` - adds `impl` block for the associated struct / enum.
- Attributes - `#[test]` - adds metadata about a item.
- function like - `vec!` - acts like a function adds code straight to the line its in.

### [`macro_rules!`](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html)

each macro must be in the following format

```rust
macro_rules! name {
    ($pattern) => {$expansion};
}
```
a pattern can contain repetitions `$(...) sep rep`

- sep can be `,;`
- rep can be `*` which indicates zero or one, and `+` for one or more repetitions.

to expand a pattern use the same format but instead 
`($a:expr),*` do `(some code with $a)*`.

## conventions

function docs should have the following sections if necessary
```rust
/// Returns an initialized serial port
/// 
/// # Arguments
///  * `a`
/// # Examples
/// ```
/// println!("this is an example");
/// ```
/// # Errors
/// ...
/// # Panics
/// ...
/// # Safety
/// ...
fn foobar() {

}
```

## *unit testing*

running `cargo test -- --show-output` will run our `#[test]` functions with their output.

### **unit testing** 
test the private interface. in each file create a module for testing.

```rust

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

```
the cfg attribute is telling the compiler to compile this code only when the *configuration* is `test` (`cargo test`). 

### **integration testing**
tests the public interface.
you need a library (src/lib.rs) to implement integration tests.

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```


## *conditional compilation*

to create a feature inside the `Cargo.toml`

```toml
[features]
default = []
# define a feature with a list (`[]`) of dependencies.
my_feature = []
```

then to add the conditional compilation just add `#[cfg(feature = "my_feature")]`

To enable a feature add it to the `default` feature.

## *type layout*

`repr(C)`

1. the type expected to be used in a C type function.
2. the type's fields expected to be reinterpret as different types.

`repr(transparent)`

1. used for one filed type, makes the type use its field representation.

## [derivable traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

## PartialEq
```
symmetric: a == b implies b == a; and
transitive: a == b and b == c implies a == c.
```
NaN != NaN, meaning it cannot be reflexive than we don't use Eq.
## Eq
```
reflexive: a == a;
symmetric: a == b implies b == a; and
transitive: a == b and b == c implies a == c.
```
## Copy
if there is no different between deep and sallow copy, it is best to use Copy trait.
which does a sallow copy.
## Clone
deep copy of an object.