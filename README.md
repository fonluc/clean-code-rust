# Clean Rust Code  

![comic](assets/clean-code-comic.jpeg)

Translated from: [https://github.com/fonluc/codigo-limpo-rust](https://github.com/fonluc/codigo-limpo-rust)

This work aims to establish the first clean code guide in Rust, providing a solid foundation for best practices in the language. However, as with any initial work, it may contain code an grammatical errors. Therefore, it is crucial for other developers to get involved and contribute to improving this repository. This will not only enrich the content but also reflect the quality and commitment of Brazilian developers, whether they are specialists in backend, Golang, Rust, web3, or blockchain. Collaboration from everyone is essential to raise the standard of software development and promote excellence in our community.

## Preface: Why Write Clean Code?  
This document serves as a reference for the Rust community, aiming to help developers write cleaner code. Whether working on a personal project or as part of a larger team, writing clean code is an essential skill. Establishing good paradigms and consistent, accessible standards for writing clean code can help prevent developers from spending hours trying to understand their own work (or that of others).

*"We don’t read code, we decode it."* - Peter Seibel

As developers, we are sometimes tempted to write code in a way that is convenient at the moment, without considering best practices; this makes code reviews and testing more difficult. In a way, we are "encoding"—and by doing so, making it harder for others to decode our work. But we want our code to be usable, readable, and maintainable. And that requires coding the right way, not the easy way.

This document starts with a simple and brief introduction to the fundamentals of writing clean code. Later, we will discuss concrete examples of refactoring specific to Rust.

### [Table of Contents](#table-of-contents)
- [Clean Rust Code](#clean-rust-code)
  - [Preface: Why Write Clean Code?](#preface-why-write-clean-code)
    - [Table of Contents](#table-of-contents)
    - [Introduction to Clean Code](#introduction-to-clean-code)
    - [Test-Driven Development\*\*](#test-driven-development)
    - [Naming Conventions](#naming-conventions)
    - [Comments](#comments)
    - [Function Naming in Rust](#function-naming-in-rust)
    - [Variable Naming in Rust](#variable-naming-in-rust)
    - [Function Cleanup in Rust](#function-cleanup-in-rust)
    - [Function Signatures in Rust](#function-signatures-in-rust)
    - [Variable Scope in Rust](#variable-scope-in-rust)
    - [Variable Declarations in Rust](#variable-declarations-in-rust)
    - [Clean Rust](#clean-rust)
    - [Returning Values](#returning-values)
    - [Returning Defined Errors](#returning-defined-errors)
    - [Returning Dynamic Errors](#returning-dynamic-errors)
    - [None Values](#none-values)
    - [Pointers in Rust](#pointers-in-rust)
    - [Mutability and Pointers](#mutability-and-pointers)
    - [Avoiding Unwanted Side Effects](#avoiding-unwanted-side-effects)
    - [Closures in Rust](#closures-in-rust)
    - [Interfaces in Go vs. Traits in Rust](#interfaces-in-go-vs-traits-in-rust)
    - [Empty Interfaces in Go vs. Dynamic and Generic Types in Rust](#empty-interfaces-in-go-vs-dynamic-and-generic-types-in-rust)

### Introduction to Clean Code  
Clean code is the pragmatic concept of promoting readable and maintainable software. Clean code builds trust in the codebase and helps minimize the chances of careless bugs being introduced. It also helps developers maintain their agility, which typically declines as the codebase expands due to the increased risk of introducing bugs.

### Test-Driven Development**

Test-driven development (TDD) is the practice of frequently testing your code throughout short development cycles or sprints. This helps maintain clean code by encouraging developers to question the functionality and purpose of their code. To facilitate testing, developers are encouraged to write short functions that do only one thing. For example, it's considerably easier to test (and understand) a 4-line function than a 40-line one.

TDD consists of the following cycle:

1. Write (or run) a test
2. If the test fails, make it pass
3. Refactor your code as needed
4. Repeat

Testing and refactoring are intertwined in this process. As you refactor your code to make it more understandable or maintainable, you need to test your changes thoroughly to ensure you haven’t altered the behavior of your functions. This can be extremely useful as the codebase grows.

### Naming Conventions

### Comments

Comments are an essential practice in programming but are often misapplied. Unnecessary comments can indicate problems in the underlying code, such as poor naming conventions. The need for a specific comment is subjective and depends on the readability of the code. Even well-written code can have complex logic that requires an explanatory comment.

In Rust, the tool `rustfmt` helps maintain a consistent style but does not dictate specific documentation rules like `gofmt` does for Go. However, Rust has a strong documentation culture through doc comments (`///` or `//!`).

It's important to distinguish between documentation comments (which start with `///` or `//!` in Rust) and other types of comments. Documentation comments should be written at a high level of abstraction, focusing more on the public interface and less on implementation details.

Other ways to explain the code include writing in a clear and expressive manner. Confusing code should not be "fixed" with explanatory comments, as this does not solve the underlying problem. Most developers tend to ignore extensive comments, and reviewing unclear code full of comments can be frustrating.

Let's look at an example of how not to comment your code in Rust:

```rust
// Iterate over the range from 0 to 9
// and invoke the function `do_something`
// for each iteration
for i in 0..10 {
    do_something(i);
}
```

This is a "tutorial comment," useful for beginners but unnecessary in production code. As experienced programmers, we should understand basic constructs like loops without needing an explanation.

Following the principle "Document the why, not the how," we can improve:

```rust
// Instantiate 10 threads to handle the future workload
for i in 0..10 {
    do_something(i);
}
```

This explains the purpose but is still not ideal. We can express this intention directly in the code:

```rust
for worker_id in 0..10 {
    spawn_thread(worker_id);
}
```

With more meaningful names, we express the intention directly in the code, making it clearer and eliminating the need for the comment.

In Rust, we can leverage language features to make the code even more expressive:

```rust
let num_workers = 10;
(0..num_workers).for_each(|worker_id| {
    spawn_thread(worker_id);
});
```

Writing clear and expressive code becomes more challenging as complexity increases. Practicing this mindset of avoiding explaining "what" the code does and focusing on "why" it is needed will result in cleaner and more maintainable code.

In Rust, use documentation comments (`///`) to document public functions and modules, leveraging the language's integrated documentation system. Reserve regular comments (`//`) for crucial explanations that cannot be expressed directly in the code.

### Function Naming in Rust

The general rule for naming functions in Rust is similar: the more specific the function, the more general its name should be. We start with broad and short function names, such as `run` or `parse`, which describe the overall functionality. For example, if we are creating a configuration parser in Rust:

```rust
fn main() {
    let config_path = std::env::args().nth(1).expect("Specify the configuration file path");

    let config = configuration::parse(&config_path).expect("Failed to parse configuration");
    
    // ...
}
```

Focusing on the `parse` function, we see that although it's short and general, the name is clear about its purpose.

At a deeper level, naming becomes a bit more specific:

```rust
fn parse(filepath: &str) -> Result<Config, Box<dyn Error>> {
    match file_extension(filepath) {
        "json" => parse_json(filepath),
        "yaml" => parse_yaml(filepath),
        "toml" => parse_toml(filepath),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown file extension")))
    }
}
```

Nested functions are clearly distinguished from their parent, allowing each to make sense both on its own and in the context of the parent.

The `file_extension` function is a bit more specific due to its nature:

```rust
fn file_extension(filepath: &str) -> &str {
    filepath.rsplit('.').next().unwrap_or("")
}
```

### Variable Naming in Rust

In Rust, just like in Go, variables should be named more specifically as we dive into nested scopes. In smaller scopes, shorter names are acceptable:

```rust
fn print_brands_in_list(brands: &[BeerBrand]) {
    for b in brands { 
        println!("{}", b);
    }
}
```

In functions with a larger scope, the distinction becomes more evident:

```rust
fn beer_brand_list_to_beer_list(beer_brands: &[BeerBrand]) -> Vec<Beer> {
    let mut beer_list = Vec::new();
    for brand in beer_brands {
        for beer in brand {
            beer_list.push(beer.clone());
        }
    }
    beer_list
}
```

### Function Cleanup in Rust

In Rust, as in Go, we keep our functions short to improve code readability. For example:

```rust
use std::error::Error;

#[derive(Default)]
struct Item;

fn get_item(ctx: &Context, json: &[u8]) -> Result<Item, Box<dyn Error>> {
    let order = Item::from_json(json)?;
    
    if !get_user_from_context(ctx).is_admin() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "User does not have sufficient privileges")));
    }
    
    db::get_item(order.item_id())
}
```

We avoid the "indentation hell" in Rust by using the `?` operator for error propagation and returning errors early. To refactor complex functions, we can break them into smaller functions:

```rust
fn get_item(extension: &str) -> Result<Item, Box<dyn Error>> {
    let reference = get_reference(extension)?;
    get_item_by_reference(&reference)
}

fn get_reference(extension: &str) -> Result<String, Box<dyn Error>> {
    db::reference_cache::get(extension)
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Reference not found in cache")))
}

fn get_item_by_reference(reference: &str) -> Result<Item, Box<dyn Error>> {
    let item = get_item_from_cache(reference)?;
    if !item.is_active() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Item is not active")));
    }
    Ok(item)
}

fn get_item_from_cache(reference: &str) -> Result<Item, Box<dyn Error>> {
    db::item_cache::get(reference)
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Item not found in cache")))
}
```

### Function Signatures in Rust

Creating a good function naming structure makes it easier to read and understand the intent of the code. Just like in Go, having functions with few input parameters improves clarity. In Rust, a recommended approach for functions with many parameters is to use structs to group these parameters.

Consider a `create_queue` function that needs many parameters:

```rust
fn create_queue(name: &str, durable: bool, delete_on_exit: bool, exclusive: bool, no_wait: bool, arguments: Option<&[(&str, &str)]>) -> Result<(), Box<dyn std::error::Error>> {
    // Function implementation
    Ok(())
}
```

Here, the `create_queue` function has many parameters. Instead, we can use a struct to represent the options:

```rust
struct QueueOptions<'a> {
    name: &'a str,
    durable: bool,
    delete_on_exit: bool,
    exclusive: bool,
    no_wait: bool,
    arguments: Option<&'a [(&'a str, &'a str)]>,
}

fn create_queue(options: QueueOptions) -> Result<(), Box<dyn std::error::Error>> {
    // Function implementation
    Ok(())
}
```

This approach improves readability and reduces the chance of errors since parameters are clearly named and organized. We can even provide default values for some of these options if necessary:

```rust
impl Default for QueueOptions<'_> {
    fn default() -> Self {
        QueueOptions {
            name: "default",
            durable: false,
            delete_on_exit: false,
            exclusive: false,
            no_wait: false,
            arguments: None,
        }
    }
}
```

### Variable Scope in Rust

Writing smaller functions helps avoid issues with mutable variables and global scope. In Rust, as in Go, global and wide-scoped variables can lead to confusion and difficult-to-debug errors.

Consider the following example

, which uses a variable with a broader scope:

```rust
fn do_complex() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Success".to_string())
}

fn main() {
    let mut val = String::new();
    let num = 32;

    match num {
        16 => {},
        32 => {
            let result = do_complex()?;
            val = result;
        },
        64 => {},
    }

    println!("{}", val);
}
```

In this example, `val` is modified within the `match`, which can lead to understanding and maintenance issues. Refactoring to limit the scope of `val` can be done as follows:

```rust
fn get_string_result(num: i32) -> Result<String, Box<dyn std::error::Error>> {
    match num {
        16 => Ok(String::new()),
        32 => do_complex(),
        64 => Ok(String::new()),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid number"))),
    }
}

fn main() {
    let val = get_string_result(32)?;
    println!("{}", val);
}
```

In this case, `val` is returned by the `get_string_result` function, and the scope of `val` is reduced.

### Variable Declarations in Rust

Declaring variables as close to their usage as possible improves code readability. In Rust, you can apply the same concept by declaring variables immediately before using them.

Consider the following example:

```rust
fn main() {
    let items = get_items();
    let sender = std::sync::mpsc::channel();
    let receiver = std::sync::mpsc::channel();

    for item in items {
        // ...
    }
}
```

We can improve clarity by creating helper functions to encapsulate the logic related to the variables:

```rust
fn create_sender() -> std::sync::mpsc::Sender<Item> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for item in receiver {
            // Process item
        }
    });
    sender
}

fn main() {
    let sender = create_sender();
    // Use the sender
}
```

By encapsulating the channel creation in a function, the declaration and usage of the `sender` variable become clearer and more isolated.

Additionally, you can create structs to encapsulate variables and provide an additional level of encapsulation and safety:

```rust
struct Sender {
    sender: std::sync::mpsc::Sender<Item>,
}

impl Sender {
    fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for item in receiver {
                // Process item
            }
        });
        Sender { sender }
    }

    fn send(&self, item: Item) {
        self.sender.send(item).expect("Failed to send item");
    }
}

fn main() {
    let sender = Sender::new();
    // Use the sender
}
```

In this example, the `sender` variable is encapsulated in a `Sender` struct, and its usage is controlled by the struct's methods.

### Clean Rust

### Returning Values

### Returning Defined Errors

In Rust, the approach to returning errors is somewhat different from Go but follows a similar principle of maintaining code readability, testability, and maintainability. Let's see how to adapt the concept of returning defined errors to Rust.

In Go, the practice of using variables for errors allows for more robust and maintainable code. In Rust, we use the `Result` enum and the `Error` trait to achieve a similar effect.

Consider a simple example of a function that returns an item from a Store:

In Go:

```go
package clean

import "errors"

var (
    NullItem = Item{}
    ErrItemNotFound = errors.New("item could not be found in the store")
)

func (store *Store) GetItem(id string) (Item, error) {
    store.mtx.Lock()
    defer store.mtx.Unlock()

    item, ok := store.items[id]
    if !ok {
        return NullItem, ErrItemNotFound
    }
    return item, nil
}
```

In Rust:

```rust
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum StoreError {
    ItemNotFound,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::ItemNotFound => write!(f, "Item could not be found in the store"),
        }
    }
}

pub struct Store {
    items: HashMap<String, Item>,
    // other fields, if any
}

impl Store {
    pub fn get_item(&self, id: &str) -> Result<Item, StoreError> {
        match self.items.get(id) {
            Some(item) => Ok(item.clone()),
            None => Err(StoreError::ItemNotFound),
        }
    }
}
```

In this example, `StoreError` is an enumeration that defines possible errors, and the `get_item` function returns a `Result<Item, StoreError>`. We use `Result` in Rust to return values that can either be a success (`Ok`) or a failure (`Err`).

### Returning Dynamic Errors

For errors that need dynamic information, Rust uses `Box<dyn std::error::Error>` to encapsulate errors that may have different types. Here's how you can adapt this:

In Go:

```go
func (store *Store) GetItem(id string) (Item, error) {
    store.mtx.Lock()
    defer store.mtx.Unlock()

    item, ok := store.items[id]
    if !ok {
        return NullItem, fmt.Errorf("Could not find item with ID: %s", id)
    }
    return item, nil
}
```

In Rust:

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DetailedError {
    message: String,
}

impl fmt::Display for DetailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DetailedError {}

pub struct Store {
    items: HashMap<String, Item>,
    // other fields, if any
}

impl Store {
    pub fn get_item(&self, id: &str) -> Result<Item, Box<dyn Error>> {
        match self.items.get(id) {
            Some(item) => Ok(item.clone()),
            None => Err(Box::new(DetailedError {
                message: format!("Could not find item with ID: {}", id),
            })),
        }
    }
}
```

Here, `DetailedError` is a struct that implements the `Error` trait, allowing for dynamic error messages. The `get_item` function returns a `Result<Item, Box<dyn Error>>`, which allows encapsulating any type of error.

### None Values

In Rust, the concept of `None` is represented by the `Option` type. The `Option` type is used for values that may be absent and is a safe way to handle the absence of values.

In Go:

```go
type App struct {
    Cache *KVCache
}

func (app *App) Cache() *KVCache {
    if app.Cache == nil {
        app.Cache = NewKVCache()
    }
    return app.Cache
}
```

In Rust:

```rust
pub struct App {
    cache: Option<KVCache>,
}

impl App {
    pub fn new() -> Self {
        App { cache: None }
    }

    pub fn cache(&mut self) -> &KVCache {
        if self.cache.is_none() {
            self.cache = Some(KVCache::new());
        }
        self.cache.as_ref().unwrap()
    }
}
```

In this example, `cache` is an `Option<KVCache>`. The `cache` method initializes the `KVCache` if it's not already present and returns a reference to the initialized `KVCache`.

In Rust, the use of `Result` and `Option` allows for more explicit control over error values and absent values. The approach of returning defined and dynamically detailed errors can be implemented using enums and custom types, improving code clarity and maintainability. Avoiding `None` values and using `Option` helps ensure that code does not end up with unhandled null values, preventing many runtime issues.

### Pointers in Rust

Pointers in Rust are treated differently than in Go, and Rust's approach to memory management and safety is more rigorous. Rust does not have the concept of null pointers as in Go, and memory safety is ensured through its type system and borrowing rules.

### Mutability and Pointers

In Rust, mutability is explicitly controlled through the use of mutable and immutable references. Mutable references (`&mut T`) allow modification of the referenced value, while immutable references (`&T`) ensure that the value cannot be changed. This approach avoids common mutability issues found in other languages.

Consider an example in Rust that reflects the use of pointers and mutability in Go:

```rust
struct User {
    id: i64,
    password: String,
}

struct UserStore {
    users: std::collections::HashMap<i64, User>,
}

impl UserStore {
    fn insert(&mut self, user: User) -> Result<(), &'static str> {
        if self.users.contains_key(&user.id) {
            return Err("Item already exists");
        }
        self.users.insert(user.id, user);
        Ok(())
    }

    fn get(&self, id: i64) -> Result<&User, &'static str> {
        self.users.get(&id).ok_or("User not found")
    }
}

fn main() {
    let mut store = UserStore {
        users: std::collections::HashMap::new(),
    };

    let user = User {
        id: 123,
        password: "secure_password".to_string(),
    };

    match store.insert(user) {
        Ok(_) => println!("User inserted successfully"),
        Err(err) => println!("Failed to insert user: {}", err),
    }

    match store.get(123) {
        Ok(user) => println!("Found user with id 123"),
        Err(err) => println!("Failed to find user: {}", err),
    }
}
```

Here, the `insert` function takes `User` by value and adds it to the `HashMap`. Since `User` is moved into the `HashMap`, the `get` function can return a reference to the stored `User` without scope issues.

### Avoiding Unwanted Side Effects

Since Rust does not allow modification of data through immutable references and enforces strict borrowing rules, common issues with unwanted side effects using pointers are mitigated.

By passing values by mutable reference, Rust ensures that only one mutable reference to a value exists at a time, avoiding conflicts:

```rust
fn update_user_password(user: &mut User, new_password: &str) {
    user.password = new_password.to_string();
}

fn main() {
    let mut user = User {
        id: 123,
        password: "old_password".to_string(),
    };

    update_user_password(&mut user, "new_secure_password");

    println!("Updated password: {}", user.password);
}
```

### Closures in Rust

Rust also has closures, which are quite similar to those found in Go but with a fundamental difference: they capture their environment safely and efficiently. Closures can capture variables from their environment in various ways (by value, by reference, or mutably), and Rust ensures that this capture is safe.

Here’s an example of using closures in Rust:

```rust
fn apply_operation<F>(data: &str, operation: F) -> String
where
    F: Fn(&str) -> String,
{
    operation(data)
}

fn main() {
    let to_uppercase = |s: &str| s.to_uppercase();
    let result = apply_operation("hello", to_uppercase);
    println!("Result: {}", result);
}
```

In the example above, `apply_operation` takes a closure that transforms the input string to uppercase. The closure captures the environment where it is defined and is passed as an argument to `apply_operation`.

### Interfaces in Go vs. Traits in Rust

1. **Implicit vs. Explicit Implementation**

In Go, an interface is implemented implicitly; if a type provides all the methods of an interface, it implements the interface without an explicit declaration. In Rust, implementing a trait is also implicit, but Rust allows explicit implementation if desired, which can help with code clarity.

Example in Rust:

```rust
trait Writer {
    fn write(&self, data: &[u8]) -> Result<usize, std::io::Error>;
}

struct NullWriter;

impl Writer for NullWriter {
    fn write(&self, data: &[u8]) -> Result<usize, std::io::Error> {
        Ok(data.len())
    }
}

fn new_null_writer() -> impl Writer {
    NullWriter
}
```

Here, `NullWriter` implements the `Writer` trait implicitly because it provides an implementation for the `write` method.

2. **Trait Implementation Verification**

While Go uses a pattern of testing to verify interface implementation, Rust does this more directly through compilation errors if the trait implementation is incorrect.

Example in Rust:

```rust
trait Writer {
    fn write(&self, data: &[u8]) -> Result<usize, std::io::Error>;
}

struct NullWriter;

impl Writer for NullWriter {
    fn write(&self, data: &[u8]) -> Result<usize, std::io::Error> {
        Ok(data.len())
    }
}

fn assert_writer_impl() {
    let _writer: Box<dyn Writer> = Box::new(NullWriter);
}
```

If `NullWriter` does not correctly implement the `Writer` trait, the code will not compile.

3. **Traits and Inheritance**

Rust does not have inheritance as in traditional object-oriented languages, but you can use traits to compose behaviors. Traits can be used to build abstractions similar to inheritance but in a more controlled and explicit manner.

Example in Rust:

```rust
trait Read {
    fn read(&self) -> String;
}

trait Write {
    fn write(&self, data: &str);
}

struct Document;

impl Read for Document {
    fn read(&self) -> String {
        "Document content".to_string()
    }
}

impl Write for Document {
    fn write(&self, data: &str) {
        println!("Writing: {}", data);
    }
}
```

4. **Trait Methods and Implementations**

Unlike Go, where method implementations might result in unexpected behaviors, Rust requires that all methods defined in a trait be implemented. Partial implementation is not allowed, which reduces the chance of confusing behavior.

Example in Rust:

```rust
trait MyReadCloser: std::io::Read + std::io::Close {}

struct MyReadCloserImpl;

impl std::io::Read for MyReadCloserImpl {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        Ok(0)
    }
}

impl std::io::Close for MyReadCloserImpl {
    fn close(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl MyReadCloser for MyReadCloserImpl {}
```

### Empty Interfaces in Go vs. Dynamic and Generic Types in Rust

1. **Empty Interfaces**

In Go, `interface{}` can store any type but often results in harder-to-maintain and error-prone code. Rust does not have a direct equivalent but offers enums and traits to create safer and less error-prone abstractions.

Example of an Enum in Rust:

```rust
enum MyEnum {
    Variant1(i32),
    Variant2(String),
}

fn process_enum(value: MyEnum) {
    match value {
        MyEnum::Variant1(n) => println!("Number: {}", n),
        MyEnum::Variant2(s) => println!("String: {}", s),
    }
}
```

2. **Generics in Rust**

Rust has robust support for generics, allowing the creation of generic types and functions in a safe and efficient manner.

Example in Rust:

```rust
struct HashMap<K, V> {
    store: std::collections::HashMap<K, V>,
}

impl<K, V> HashMap<K, V> {
    fn new() -> Self {
        HashMap {
            store: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }
}
```