# Rust Notes

Pretty much a summary of <https://doc.rust-lang.org/book/>.

- functional programming language
  - `main.main` (main.rs) is the main function
- print with `println!` and `print!` functions
- `let` declares variables
  - `mut` makes variables mutable
  - colons used for assigning types
- loops are by default infinite
  - break and continue to exit and skip
  - while loops also exist
  - for loops need to be made manually (the `start..end` syntax is available)
  - loops can be named for specifying which loop to break and continue
  - loops can return values
- `struct` is similar to `interface` in typescript (used for typing objects)
  - e.g. `struct Point { x: i32, y: i32 }` and `let p = Point { x: 10, y: 10 }`
  - `..object` uses the fields from the given instance to fill in the rest
  - tuple structs can be used to make a struct without named fields (`struct Point(i32, i32, i32);` and `let p = Point(1, 2, 3);`)
  - unit-like structs act similarly to a single field on an enum and are always equal to themselves (`struct AlwaysEqual;` and `let subject = AlwaysEqual;`)
  - structs can have implementation blocks (`impl Rectangle {}`)
- methods are functions found in structs, enums, or traits
  - e.g.

    ```rust
    struct Rectangle {
      width: u32,
      height: u32,
    }

    impl Rectangle {
      fn area(&self) -> u32 {
        self.width * self.height
      }
    }
    ```
  
  - the first parameter of a method is always a reference to the parent
- `enum` is the same as in typescript
  - e.g. `enum Direction { Up, Down, Left, Right }` and `let direction = Direction::Up` or `fn move(dir: Direction) {}`
  - enums can be associated data types

    ```rust
    enum Direction { 
      Up(i32), 
      Down(i32), 
      Left(i32), 
      Right(i32) 
    }
    ```

  - enums can also have multiple types associated with them

    ```rust
    enum Direction {
      Up(i32),
      Down(i32),
      Left(i32),
      Right(i32),
      UpLeft(i32, i32),
      UpRight(i32, i32),
      DownLeft(i32, i32),
      DownRight(i32, i32),
    }

  - enums can also have multiple named types associated with themselves

    ```rust
    enum Direction {
      Up(i32),
      Down(i32),
      Left(i32),
      Right(i32),
      UpLeft{ up: i32, left: i32 },
      UpRight{ up: i32, right: i32 },
      DownLeft{ down: i32, left: i32 },
      DownRight{ down: i32, right: i32 },
    }
    ```

  - enums can alo have implementations similarly to structs

    ```rust
    enum Direction {
      Up(i32),
      Down(i32),
      Left(i32),
      Right(i32),
    }

    impl Direction {
      fn is_up(&self) -> bool {
        match self {
          Direction::Up(_) => true,
          _ => false,
        }
      }
    }
    ```

  - enums can also have options

    ```rust
    enum Direction<T> {
      Up(T),
      Down(T),
      Left(T),
      Right(T),
      UpLeft{ up: T, left: T },
      UpRight{ up: T, right: T },
      DownLeft{ down: T, left: T },
      DownRight{ down: T, right: T },
    }
    ```

    - now the values are of type `T`

    - `Option<T>` replaces something like `null` in javascript

      ```rust
      enum Option<T> {
          None,
          Some(T),
      }
      ```

    - `Option<T>` is included in the standard library

- `match` is similar to `switch` in javascript but it can return a value

  ```rust
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }
  ```

  - fellow englishmen I agree the American coin system makes no sense

  - `match` can also take the value from the enum

    ```rust
    enum UsState {
      Alabama,
      Alaska,
      // ...
    }

    enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
      match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
          println!("State quarter from {:?}!", state);
          25
        }
      }
    }
    ```

  - `match` with `Option<T>`

    ```rust
    fn plus_one(n: Option<i32>) -> Option<i32> {
      match n {
        None => None,
        Some(i) => Some(i + 1),
      }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    ```

  - the `other` pattern is like the `default` pattern in javascript

    ```rust
    let n = 10;
    println!(
      match n {
        3 => 1,
        7 => 3,
        other => other,
      }
    )
    ```

    - here `10` is printed

  - the `_` pattern is like the `other` pattern but it doesn't bind the value

    ```rust
    let n = 10;
    println!(
      match n {
        3 => 1,
        7 => 3,
        _ => 7,
      }
    )
    ```

    - here `7` is printed
- `if let` combines `if` and `let`
  - e.g. it simplifies the following code

    ```rust
    let some_option = Some(3);
    match some_option {
      Some(amount) => println!("number {}!", amount),
      _ => (),
    }
    ```

    to

    ```rust
    let some_option = Some(3);
    if let Some(amount) = some_option {
      println!("number {}!", amount);
    }
    ```

  - `if let` can also be used to match on multiple values
    - e.g.

    ```rust
    let some_option = Some(3u8);
    match some_option {
      Some<u8>(amount) => println!("number (8bit) {}!", amount),
      Some<u32>(amount) => println!("number (32bit) {}!", amount),
      _ => (),
    }
    ```

    to

    ```rust
    let some_option = Some(3);
    if let Some<u8>(amount) = some_option {
      println!("number (8bit) {}!", amount);
    } else if let Some<u32>(amount) = some_option {
      println!("number (32bit) {}!", amount);
    }
    ```

- `Vec<T>` (aka vectors) is a collection type or list
  - in rust like most other languages, the first element in a list is at index 0
  - `Vec` is simply a growable array

  - create an empty vector

    ```rust
    let v: Vec<i32> = Vec::new();
    ```

  - create a vector with some values

    ```rust
    let v = vec![1, 2, 3];
    ```

  - create a vector and add values dynamically

    ```rust
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    ```

  - get an item from a vector

    ```rust
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    ```

  - iterate over a vector

    ```rust
    let v = vec![100, 32, 57];
    for i in &v {
      println!("{}", i);
    }
    ```

  - get an item that might not exist from a vector

    ```rust
    let v = vec![100, 32, 57];
    let does_not_exist = &v[100];
    ```

    - this will cause an error

    - instead use a `get` method

      ```rust
        let v = vec![100, 32, 57];
        let does_not_exist = v.get(100);
      ```

  - iterate over a vector and mutate the values

    ```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
      *i += 50;
    }
    ```

  - storing multiple values with enums

      ```rust
      enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
      }

      let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
      ];
      ```

  - create a vector and remove values dynamically

    ```rust
    let mut v = Vec![0, 1, 2, 3, 4, 5];
    v.pop();
    v.pop();
    v.pop();
    ```

    - `pop` also returns the value that was removed

- `HashMap<K, V>` is like an object in javascript (it stores key-value pairs)
  - `HashMap` is a collection type
  - you must always import `HashMap` before using it

    ```rust
    use std::collections::HashMap;
    ```

  - create an empty hashmap

    ```rust
    let mut scores = HashMap::new();

    println!("{:?}", scores);
    ```

  - create a hashmap with some values

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    ```

  - create a `HashMap` from iterables

    ```rust
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
    ```

  - get a value from a hashmap

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{}", score);
    ```

  - iterate over a hashmap

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }

    println!("{:?}", scores);
    ```

  - iterate over a hashmap and mutate the values

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &mut scores {
      *value += 10;
    }

    println!("{:?}", scores);
    ```

  - overwrite a value in a hashmap

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    ```

  - only insert a value if the key does not already exist

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    ```

  - update a value based on the old value

    ```rust
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }

    println!("{:?}", map);
    ```

- `panic!` throws an exception and stops the program

  ```rust
  panic!("crash and burn");
  ```

- `Result<T, E>` is the type used if something can be an error or a value

  ```rust
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

  - `Ok` is a successful result

    ```rust
    let x = Ok(5);
    ```

  - `Err` is an error result

    ```rust
    let x: Result<i32, &str> = Err("Error");
    ```

  - `Result<T, E>` is included in the standard library
  - `match` statements are used to handle results

    ```rust
    let x = Ok(5);
    let y = Err("Error");

    match x {
      Ok(val) => println!("success {}", val),
      Err(e) => println!("error {}", e),
    }

    match y {
      Ok(val) => println!("success {}", val),
      Err(e) => println!("error {}", e),
    }
    ```

  - match on different errors

    ```rust
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
      let f = File::open("hello.txt");

      let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
          },
          other_error => {
            panic!("Problem opening the file: {:?}", other_error)
          }
        },
      };
    }
    ```

  - the `expect` function is used to handle errors

    ```rust
    use std::fs::File;

    fn main() {
      let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    ```

  - Propagate errors

    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
      let f = File::open("hello.txt");

      let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
      };

      let mut s = String::new();

      match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
      }
    }
    ```

  - Propagate errors with `?` (this does the same as above)

    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
    }
    ```

    or even shorter

    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
      let mut s = String::new();
      File::open("hello.txt")?.read_to_string(&mut s)?;
      Ok(s)
    }
    ```

    - `?` can only be used if the return type is `Result`, `Option` or another type that implements `std::ops::Try`

    - `?` can be used in a `for` loop

      ```rust
      use std::fs::File;
      use std::io::{self, Read};

      fn main() {
        for line in File::open("hello.txt")?.lines() {
          let line = line?;
          println!("{}", line);
        }
      }
      ```

    - `?` can be used in a `while` loop

      ```rust
      use std::fs::File;
      use std::io::{self, Read};

      fn main() {
        let mut f = File::open("hello.txt")?;

        let mut buffer = String::new();
        while f.read_to_string(&mut buffer)? > 0 {
          println!("{}", buffer);
          buffer.clear();
        }
      }
      ```

    - `?` can be used in a `match` statement

      ```rust
      use std::fs::File;
      use std::io::{self, Read};

      fn main() {
        let f = File::open("hello.txt")?;

        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
          Ok(_) => println!("{}", buffer),
          Err(e) => println!("Error: {}", e),
        }
      }
      ```

    - `?` can be used in a `let` statement

      ```rust
      use std::fs::File;
      use std::io::{self, Read};

      fn main() {
        let f = File::open("hello.txt")?;

        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        println!("{}", buffer);
      }
      ```

    - the main function can be given another return type to get around this

      ```rust
      use std::error::Error;
      use std::fs::File;

      fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
      }
      ```

- `trait` can be used to make a list of functions an object must have, this is a bit similar to inheritance in javascript [example](/object_oriented_gui)
  - this makes it so that we can have a group of different types that can be used under  the same name
  - `impl` can be used to implement a trait for an object

    ```rust
    trait Draw {
      fn draw(&self);
    }

    struct Button {
      width: u32,
      height: u32,
      label: String,
    }

    impl Draw for Button {
      fn draw(&self) {
        // ...
      }
    }

    struct Paragraph {
      width: u32,
      height: u32,
      text: String,
    }

    impl Draw for Paragraph {
      fn draw(&self) {
        // ...
      }
    }
    ```

    this is pretty much equivalent to the following javascript

    ```javascript
    class Draw {
      draw() {}
    }

    class Button extends Draw {
      constructor(width, height, label) {
        this.width = width;
        this.height = height;
        this.label = label;
      }

      draw() {
        // ...
      }
    }

    class Paragraph extends Draw {
      constructor(width, height, text) {
        this.width = width;
        this.height = height;
        this.text = text;
      }

      draw() {
        // ...
      }
    }
    ```

  - so long as a type implements the trait it can be used under the trait as its type name

    ```rust
    pub struct Screen {
      pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
      pub fn run(&self) {
        for component in self.components.iter() {
          component.draw();
        }
      }
    }

    // as you can see the Paragraph and Button types can be used under the Draw trait
    let screen = Screen {
      components: vec![
        Box::new(Paragraph {
          width: 75,
          height: 10,
          text: String::from("Hello World!"),
        }),
        Box::new(Button {
          width: 50,
          height: 10,
          label: String::from("OK"),
        }),
      ],
    };

    screen.run();
    ```

- `std::thread`, similar to python's `threading` module, allows you to spawn a new thread to run asynchronously

  ```rust
  use std::thread;
  use std::time::Duration;

  fn main() {
    thread::spawn(|| {
      for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
      }
    });

    for i in 1..5 {
      println!("hi number {} from the main thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  }
  ```

  - it is important to note that the thread is spawned in the background, so the main thread will continue to run and the spawned thread will stop running when the main thread is done
    - so the output from the example will be

      ```log
      hi number 1 from the main thread!
      hi number 1 from the spawned thread!
      hi number 2 from the main thread!
      hi number 2 from the spawned thread!
      hi number 3 from the main thread!
      hi number 3 from the spawned thread!
      hi number 4 from the main thread!
      hi number 4 from the spawned thread!
      ```

      although you would expect

      ```log
      hi number 1 from the main thread!
      hi number 1 from the spawned thread!
      hi number 2 from the main thread!
      hi number 2 from the spawned thread!
      hi number 3 from the main thread!
      hi number 3 from the spawned thread!
      hi number 4 from the main thread!
      hi number 4 from the spawned thread!
      hi number 5 from the spawned thread!
      hi number 6 from the spawned thread!
      hi number 7 from the spawned thread!
      hi number 8 from the spawned thread!
      hi number 9 from the spawned thread!
      ```

  - wait for the spawned thread to finish

    ```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
      let handle = thread::spawn(|| {
        for i in 1..10 {
          println!("hi number {} from the spawned thread!", i);
          thread::sleep(Duration::from_millis(1));
        }
      });

      for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
      }

      handle.join().unwrap();
    }
    ```

    - now the output will be

      ```log
      hi number 1 from the main thread!
      hi number 1 from the spawned thread!
      hi number 2 from the main thread!
      hi number 2 from the spawned thread!
      hi number 3 from the main thread!
      hi number 3 from the spawned thread!
      hi number 4 from the main thread!
      hi number 4 from the spawned thread!
      hi number 5 from the main thread!
      hi number 5 from the spawned thread!
      hi number 6 from the spawned thread!
      hi number 7 from the spawned thread!
      hi number 8 from the spawned thread!
      hi number 9 from the spawned thread!
      ```

  - use the `move` keyword to move ownership of a value into a new thread

    ```rust
    let v = vec![1, 2, 3];
    let t = thread::spawn(|| { println!("{:?}", v); });
    drop(v);
    t.join().unwrap();
    ```

    - this will cause an error because `v` might not be valid when it is needed in the thread, instead add the `move` keyword to the thread

      ```rust
      let v = vec![1, 2, 3];
      let t = thread::spawn(move || { println!("{:?}", v); });
      drop(v);
      t.join().unwrap();
      ```

      - this will cause an error because `v` is being dropped before the thread is finished and the ownership of `v` has been moved into the thread

  - > “Do not communicate by sharing memory; instead, share memory by communicating.”
    <https://golang.org/doc/effective_go#concurrency>
    - this is a good example of why you should not communicate by sharing memory, instead communicate by communicating

        ```rust
        use std::thread;
        use std::time::Duration;

        fn main() {
          let v = vec![1, 2, 3];
          let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
          });
          handle.join().unwrap();
        }
        ```

      - this will cause an error because `v` is being dropped before the thread is finished and the ownership of `v` has been moved into the thread
    - instead of sharing variables between threads, pass messages between threads

      ```rust
      use std::sync::mpsc;
      use std::thread;

      fn main() {
        let m = mpsc::channel();
        let s = mpsc::channel();

        let spawned = thread::spawn(move || {
          let val = String::from("ping");
          m.0.send(val).unwrap();
          let received = s.1.recv().unwrap();
          println!("Got: {}", received);
        });

        let received = m.1.recv().unwrap();
        println!("Got: {}", received);
        let val = String::from("pong");
        s.0.send(val).unwrap();
        
        spawned.join().unwrap();
      }
      ```

      - send multiple messages and iterate over them

        ```rust
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
          let (tx, rx) = mpsc::channel();

          thread::spawn(move || {
            let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("thread"),
            ];

            for val in vals {
              tx.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
            }
          });

          for received in rx {
            print!("{} ", received);
          }
        }
        ```

      - multiple producers

        ```rust
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
          let (tx, rx) = mpsc::channel();

          let tx1 = tx.clone();
          thread::spawn(move || {
            let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("thread"),
            ];

            for val in vals {
              tx1.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
            }
          });

          thread::spawn(move || {
            let vals = vec![
              String::from("more"),
              String::from("messages"),
              String::from("for"),
              String::from("you"),
            ];

            for val in vals {
              tx.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
            }
          });

          for received in rx {
              println!("Got: {}", received);
          }
        }
        ```

- to import a local module first use the mod key word followed by the name of the module

  ```rust
    mod my_mod;
    use my_mod::hello;
  ```

- `std::env` includes the `args` function to get the args and the `var` function to get an environment variable

- `eprintln!` and `eprint!` are macros that print to stderr

- templates can be used to make code run with multiple types

  - `struct`

    ```rust
    struct Message<T> {
      content: T,
      from: String,
      to: String,
    }

    fn main() {
      let x = Message::<String> {
        content: "hello".to_string(),
        from: "Boris".to_string(),
        to: "Joe".to_string(),
      };
      let y = Message::<u32> {
        content: 123u32,
        from: "Joe".to_string(),
        to: "Boris".to_string(),
      };
    }
    ```

  - `fn`

    ```rust
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];

      for item in list.clone() {
        if &item > &largest {
          largest = &item;
        }
      }

      largest
    }

    fn main() {
      let number_list = vec![34, 50, 25, 100, 65];

      let result = largest(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];

      let result = largest(&char_list);
      println!("The largest char is {}", result);
    }
    ```

    - using `+`

      ```rust
      fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
          if item > largest {
            largest = item;
          }
        }

        largest
      }

      fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
      }
      ```

  - `enum`

    ```rust
    enum Result<T, E> {
      Ok(T),
      Err(E),
    }
    ```

  - `impl`

    ```rust
    struct Point<T> {
      x: T,
      y: T,
    }

    impl<T> Point<T> {
      fn x(&self) -> &T {
        &self.x
      }
    }

    fn main() {
      let p = Point { x: 5, y: 10 };

      println!("p.x = {}", p.x());
    }
    ```
