**Printing**
`println!("Hello world!");`

**Declaring a variable**
```let x : u32 = 65;```

**Declaring a mutable string**
```let mut text = String::from("hello");```

**Calling a function with a mutable reference to 'string'**
```function(&mut string);```

**Define a constant**
```pub const MAX_SIZE: i32 = 100; ```

**Define variable of type `u8` and value `42`**
```
let _x: u8 = 42;
**Define variable of type `f64` and value `3.14`**
let _y: f64 = 3.14;
**Define variable of type `bool` and value `false`**
let _logical: bool = false;
**Define variable of type `char` and value `a`**
let _a: char = 'a';
**Return a tuple with the variables in the order they were defined**
let my_tuple = (42, 3.14, false, 'a');
my_tuple
```

**Conversion**
```
pub fn numerical_type_conversion(n: i32) -> u32 {
    // Implement here
    let y: u32 = n as u32;
    y
    or
    n as u32
}
```

**Sum of an array**
```
pub fn sum_array(arr: &[i32]) -> i32 {
    let sum: i32 = arr.iter().sum();
    sum
}
```

**Tuples**
```
pub fn create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String) {
    // TODO: Implement the function here
    return (a,b,c.to_string());    
}
```


**Return a string**
```
pub fn check_number_sign(number: i32) -> String {
    if number < 0 {
        "negative".to_string()
    } else if number > 0 {
        "positive".to_string()
    } else {
        "zero".to_string()
    }    
}
```

**Character counting string**
```
pub fn count_characters(s: &str) -> u32 {
    // Count the number of characters in the string
    s.chars().count() as u32    
}
```

**Slice**
```
pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {    
    if slice.is_empty() {
        return None
    }    
    slice.iter().max().copied()    
}
```

**Mutable Slices**
```
pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for i in slice {
        if *i % 2 == 0{
            *i = *i * 2
        }else{
            *i = *i - 1 
        }
    }
}
```
Since i is a reference:

*i → actual value
You must dereference to read or modify

**Slice Manipulation**
```
pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    for &i in indices{
        if i<slice.len(){
            slice[i]=value;
        }
    }
}
```
&i is used to dereference while iterating, so you get the value instead of a reference

**Structs**
```
pub struct Person {
    // Define fields here
    // Read the description
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    if person.age >= 18{
        true
    }else{
        false
    }  
}
```
**Tuple Structs**

```
pub struct Rectangle(pub f32, pub f32); // 1. Finish the struct
pub fn area(rect: &Rectangle) -> f32 {
    return rect.0*rect.1;  
}
```
**Constructors**
```
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i32 ) -> Book{
        Book{
            title:title.to_string(),
            author:author.to_string(),
            year:year,
            likes: 0
        }
    } 
}

```

**Unit Structs**
```
// Define a struct named `Logger`
// Implement an associated function `log_message`
// That accepts a `&str` and prints the output.

pub struct Logger;
impl Logger{
    pub fn log_message(message: &str){
        print!("[LOG]: {message}");
    }
} 

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}
```

**Methods on Structs**
```
// 1. Define the struct
pub struct Counter{
    count: i32, 
}

// 2. Implement the associated function and methods
impl Counter{
    pub fn new() -> Self{
        Self{count: 0}
    }
    pub fn increment(&mut self){
        self.count += 1;
    } 
    pub fn decrement(&mut self){
        self.count -= 1;
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }
}
```

**Enums**
```
pub enum TrafficLight {
    // Define enum variants here
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    // Your code here...
    match light{
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}
```

**Playing Cards**
```
pub enum Card {
    // Define the Card variants here
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    // Your code here...
        match card {
        Card::King => String::from("King"),
        Card::Queen => String::from("Queen"),
        Card::Jack => String::from("Jack"),
        Card::Numbered(value, suit) => format!("{value} of {suit}"),
    }
}
```

**Complex Enums**
```
pub enum Animal {
    // Define the Animal variants here
    Dog,
    Cat(String),
    Bird{ species: String, can_fly: bool }
}

pub fn describe_animal(animal: &Animal) -> String {
    // Your code here...
    match animal{
        Animal::Dog => String::from("A friendly dog."),
        Animal::Cat(name) => format!("A cat named {name}."), 
        Animal::Bird {species, can_fly} => if *can_fly {format!("A {species} that can fly.")} else {format!{"A {species} that cannot fly."}} 
    }
}
```

**Methods on Enums**
```
pub enum VehicleStatus {
    // Define the VehicleStatus variants here
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        // Your code here...
        match self{
            VehicleStatus::Parked => true,
            VehicleStatus::Driving{speed:_} => true,
            _ => false
        }
    }

    pub fn description(&self) -> String {
        // Your code here...
        match self {
            VehicleStatus::Parked => String::from("The vehicle is parked."),
            VehicleStatus::Driving { speed } => format!("The vehicle is driving at {speed} km/h."),
            VehicleStatus::BrokenDown(reason) => format!("The vehicle is broken down: {reason}."),
        }
    }
}
```



