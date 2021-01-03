# ch9 Error Handling

rust没有异常。相反，rust将错误分成两类：*recoverable*和*unrecoverable*。对于可恢复错误，rust利用`Result<T, E>`类型对其进行处理。对于不可恢复错误，rust通过`!panic`宏终止程序的执行。

## Unrecoverable Errors with `panic!`

默认情况下，当panic时，rust回顾整个栈，并依此清除遇到的每个函数的数据，因此需要花费相当的时间。如果需要让生成的二进制文件尽可能小，可以选择直接abort，并让操作系统回收资源，可以在*Cargo.toml*文件中的`[profile]`节增加
```yaml
[profile.release]
panic = 'abort'
```
### 直接使用`panic!`宏

```rust
fn main() {
    panic!("hello");
}
```

### 间接使用`panic!`宏

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

+ c中*buffer overread*是未定义行为，并能导致内存安全问题
+ rust会导致panic，通过`RUST_BACKTRACE`能够打印trace
    + 自顶向下浏览直到自己写的函数
    + 其之上是自己代码所调用的，其之下是调用自己代码的
+ 为了获取trace需要使能debug，当使用`cargo build`或`cargo run`在没有`--release`标记时，默认是debug模式。

## Recoverable Errors with `Result`

### Result枚举类型

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> `Option`和`Result`均在prelude中，因此不需要在`Ok`和`Err`前指明`Result::`

### 处理不同类型`Errors`

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
        }
    };
}
```

使用闭包取代过多的match

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opending the file: {:?}", error);
        }
    })
}
```

### panic on Error: 使用unwrap和expect

`Result<T, E>`类型上定义的方法可用于减少match的使用。

+ `unwrap()`: 当`Result`值为`Ok`时，返回其内包含的值。`Err`时，调用`panic!`宏。
+ `expect()`: 和`unwrap()`类似，但是可以选择`panic!`宏的错误信息。

### propagating Errors

返回error给函数的调用者，而非在函数内部处理。

#### 使用`?`缩写传播Errors

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?`运算符在遇到error值时会执行`from`函数(由标准库的`From`trait定义，用于将error从一个类型转换为另一个类型)，即将error类型转换为当前函数返回值的error类型。因此只要实现了`From`trait，多种error类型都可以直接转换后返回。

一种更简便的实现如下：

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

或者这样：

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

当调用返回值类型为`Result<T, E>`类型的函数时，若希望能够使用`?`，则要么将当前函数返回值类型也改为`Result<T, E>`，要么使用`match`进行处理。

`main`函数较为特殊，其返回值存在限制。一种有效的返回值是`()`，另一种是`Result<T, E>`。因此可以这么写：

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

`Box<dyn Error>`类型是一个trait对象，可用于表示任意error。

## To `panic!` or Not to `panic`

使用`panic`时，相当于为调用者做出决定。而返回`Result`，调用者则可以自行做出决定。大多数情况下，返回`Result`是一个好的选择。

### 在examples, Prototype Code和Tests中使用`panic`是合适的

在examples和prototype中使用`unwrap()`或者`expect()`可视为placeholder。可在之后完善。

test中将`panic!`视为error，因此可以直接使用`unwrap()`和`expect()`。

### using `panic` when you have more information than the compiler

```rust
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

显然，此处"127.0.0.1"不存在故障问题。

### error handling guidelines

当代码可能进入bad state时，使用panic是合适的。bad state指的是程序中某些假设、保证、协议或不变式被打破，例如：

+ 无效值，自相矛盾的值，缺失的值
+ 并非预期偶尔会发生的情况
+ 之后的代码依赖于不会进入bad state
+ 不存在合适的方式将该信息编码进使用的类型

如果函数调用者传入无意义的值，调用`panic!`是最好的选择。

然而，如果是预期可能发生的故障，或者缺少如何处理的上下文，则使用`Result`更合适。例如：HTTP请求返回表明达到速率界限的状态码。向parser传入的格式错误的数据。

当对值执行操作，应当首先验证值的有效性。若无效，则panic。例如：对内存的越界访问。调用函数通常需要满足协议(contract)，若协议被违反，使用panic是合理的，因为这通常表明是一个调用者端的bug。

#### rust类型系统的帮助

+ 如果参数类型是明确的，则编译器能够确保值是有效的。即，如果类型并非Option，则无须考虑None。或者类型是u32，则参数不可能是复数。

### creating custom types for validation

验证参数合法性的一种方式是在使用之前用`if`语句判断有效性。然而如果有多个函数都有此要求，则验证代码会变得十分冗余。

相反可以使用一个新类型，并将验证代码放入该类型的创建方法中，而不是在每个地方都重复验证。

```rust
mod my {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
```

上面代码就保证了Guess类型的value在\[1,100\]的范围内。如果传入的值不满足该要求，则使用panic，同时需要在API文档中指明。模块以外的代码必须使用`new`方法初始化，因为`Guess`各个域是私有的(对模块外部代码而言)。
