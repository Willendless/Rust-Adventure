# ch6 enums and pattern matching

## Enum的使用

### 1. 表示不同类别的数据

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
```

+ 同一个enum类型的不同变种归属于类型标识符的命名空间(namespaced under its identifier)。

### 2. 表示不同类别数据及其关联数据

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr2::V4(127, 0, 0, 1);
```

#### 标准库对ip地址数据的结构体定义

```rust
struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

### 3. 表示不同类型的struct

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct QuitMessage; // unit struct
// struct MoveMessage {x: i32, y: i32,};
// struct WriteMessage(String) // tuple struct
// struct ChangeColorMessage(i32, i32, i32) // tuple struct
```

+ 这样可以定义单个方法处理所有变种。

```rust
impl Message {
    fn call(&self) {
        // something here
    }
}
```

### Option Enum类型

+ `Option`enum类型由标准库定义，作用在于使用一个特殊类型表示null值的含义，而非直接使用null。
+ `Option`enum类型包含在prelude中，因此当使用`Some`和`None`时，不需要`Option::`前缀

```rust
enum Option<T> {
    Some<T>,
    None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

+ 对于Some变种，编译器能够推测出T的类型。对于None，需要指明T的类型。

#### 目的

+ 目的：区分数据类型和null类型
+ 对于具体数值类型，例如`i8`，rust编译器能够保证其内为有效数据。只有在Option<i8>的情况下，才需要考虑无效数据的情况。

> 当希望执行T的操作时，需要首先将`Option<T>`类型转换为`T`类型，这帮助了编译器捕获null带来的问题：将是null的值当成非null。  

总的来说，为使用`Option<T>`的值，需要提供处理不同变种情况的代码。而这由`match`控制流表达式提供。

## `match` 控制流操作符

+ match操作符将数据值和一系列模式进行匹配，并基于匹配结果执行该模式对应代码。
+ 模式可以由字面量、变量名、通配符等构成，而if语句只能基于bool类型值进行选择。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: coin) -> u8 {
    match coin {
        Coin::Penny => {
            // do something here
            1,
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### match操作符arm的值绑定

+ match arm能够将匹配的模式对应的值绑定到arm变量上。

```rust
struct t {x: u32, y: u32};
enum Coin {
    Penny,
    Nickel(t),
}

match coin {
    Coin::Penny => 1,
    Coin::Nickel(x, y) => {
        // do something here
    }
}
```

### 匹配Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
```

> 1. `match`表达式是exhaustive的，这意味着必须穷举每一种pattern情况。
> 2. `_`是`match`表达式的通配符，类似于`switch`中的`default`子句。

## `if let`控制流

+ `if let`语法是一种特殊的match表达式，主要用于只有一种匹配情况下需要执行相关代码的情况。
+ `if let`需要提供一种模式和一个表达式，它们之间用等号分隔。类似于match，其中的模式即第一个arm。

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("Three"),
    _ => (),
}
// 等价的if let结构
if let Some(3) = some_u8_value {
    println!("three");
} else {
    // might do something else
}
```
