# Rust Notes

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
      n = 10;
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
      n = 10;
      println!(
        match n {
          3 => 1,
          7 => 3,
          _ => 7,
        }
      )
      ```

      - here `7` is printed
