// clean-code.rust

// Example of how not to comment your code in Rust:
fn example_1() {
    // Iterate over the range from 0 to 9
    // and invoke the function `do_something`
    // for each iteration
    for i in 0..10 {
        do_something(i);
    }
}

// Following the principle "Document the why, not the how", we can improve:
fn example_2() {
    // Instantiate 10 threads to handle the future workload
    for i in 0..10 {
        do_something(i);
    }
}

// This explains the purpose, but it's still not ideal. We can express this intention directly in the code:
fn example_3() {
    for worker_id in 0..10 {
        spawn_thread(worker_id);
    }
}

// In Rust, we can leverage the language features to make the code even more expressive:
fn example_4() {
    let num_workers = 10;
    (0..num_workers).for_each(|worker_id| {
        spawn_thread(worker_id);
    });
}

// Example of reading and setting up files:
fn main() {
    let config_path = std::env::args().nth(1).expect("Specify the configuration file path");

    let config = configuration::parse(&config_path).expect("Failed to parse configuration");
    
    // ...
}

// Function to parse configurations based on file extension:
fn parse(filepath: &str) -> Result<Config, Box<dyn Error>> {
    match file_extension(filepath) {
        "json" => parse_json(filepath),
        "yaml" => parse_yaml(filepath),
        "toml" => parse_toml(filepath),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown file extension")))
    }
}

// Helper function to get the file extension:
fn file_extension(filepath: &str) -> &str {
    filepath.rsplit('.').next().unwrap_or("")
}

// Function to print beer brands:
fn print_brands_in_list(brands: &[BeerBrand]) {
    for b in brands { 
        println!("{}", b);
    }
}

// Function to convert a list of beer brands into a list of beers:
fn beer_brand_list_to_beer_list(beer_brands: &[BeerBrand]) -> Vec<Beer> {
    let mut beer_list = Vec::new();
    for brand in beer_brands {
        for beer in brand {
            beer_list.push(beer.clone());
        }
    }
    beer_list
}

// Example function to get an item from the database:
fn get_item(ctx: &Context, json: &[u8]) -> Result<Item, Box<dyn Error>> {
    let order = Item::from_json(json)?;
    
    if !get_user_from_context(ctx).is_admin() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "User does not have sufficient privileges")));
    }
    
    db::get_item(order.item_id())
}

// Example functions to get an item based on a reference:
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

// Example of creating a queue with configurable options:
fn create_queue(name: &str, durable: bool, delete_on_exit: bool, exclusive: bool, no_wait: bool, arguments: Option<&[(&str, &str)]>) -> Result<(), Box<dyn std::error::Error>> {
    // Function implementation
    Ok(())
}

// Example of a struct and function to create a queue with options:
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

// Implementing the `Default` trait for `QueueOptions`:
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

// Example of a function that returns a result based on a number:
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

// Example of a function that returns a string based on a number:
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

// Example of using channels in Rust:
fn main() {
    let items = get_items();
    let (sender, receiver) = std::sync::mpsc::channel();

    for item in items {
        // ...
    }
}

// Function to create a sender and receive items:
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

// Struct and implementation of `Sender` to send items:
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

// Example of error handling in Go:
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

// Example of error handling in Rust:
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

// Example of detailed error handling in Rust:
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
    pub fn get_item(&self, id: &str) -> Result<Item, DetailedError> {
        match self.items.get(id) {
            Some(item) => Ok(item.clone()),
            None => Err(DetailedError {
                message: format!("Item with ID '{}' not found in the store", id),
            }),
        }
    }
}
