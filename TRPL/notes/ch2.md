# Programming a Guessing Game

## 分析I

+ 默认情况下，rust从`prelude`将任意程序所需类型的最小集合引入程序的作用域(scope)。如果所需的类型不在prelude中，则需要使用`use`语句引入，例如`use std::io;`。

### 函数体

+ 使用字符串：String是utf-8编码，可增长的字符串类型。
+ 读取用户输入：`io::stdin()`函数返回`std::io::Stdin`类型实例，即标准输入的句柄`io::stdin().read_line(&mut guess)`从标准输入读取行

### 利用Result类型处理潜在错误

+ `std::result::Result`是泛型枚举类型，包含Ok和Err两个变种(variants)。以`read_line()`的返回`io::Result`值为例，Ok中包含了成功生成的值，而Err包含了错误信息。
+ Result的`expect()`方法用于错误处理。若Result实例是Err值，则会终止程序并打印`expect()`参数，若实例是Ok，则返回Ok内部的值。`read_line()`返回的Ok值包含了用户键入标准输入的字节数
+ 若不想程序终止，则需要自己编写错误处理逻辑，而不用调用`expect()`

### 打印信息

+ `println!("{}")`，`{}`占位符类似于蟹钳包裹住其内的值。

## 分析II

### 使用随机数

#### crate与cargo

+ *crate*: rust源代码文件的集合。
+ 从project编译构建得到的crate是*binary crate*，即可执行crate。
+ *rand crate*是*library crate*，其内包含了用于其它程序的代码。
+ 在cargo.toml的`[dependencies]`下加入`rand="0.5.5"`，"0.5.5"是语义版本标识符，指的是“任何和0.5.5版本公共API兼容的版本”。
+ 第一次执行`cargo build`时会生成Cargo.lock文件，同时将dependencies中的所有版本信息写入该文件

#### 随机数生成

+ 当需要使用随机数生成相关的方法时，需要导入Rng trait，该trait定义了实现随机数生成器的方法，`use rand::Rng`
+ `rand::thread_rng()`返回一个具体的随机数生成器
+ `rand::thread_rng().gen_range(1, 101)`中`gen_range()`方法由`Rng`trait定义，生成一个以所给参数为边界的随机数

> `cargo doc --open`命令为所有本地依赖编译doc文档。

### 使用match比较

+ 导入`std::cmp::Ordering;`枚举类型，包含`Less`,`Greater`,`Equal`三个变种。
+ match表达式由arm组成，每一arm表示一种pattern，rust依次匹配每一arm。

### 字符串和整数转换

```rust
let guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
let guess: u32 = guess.trim().parse().expect("Please type a number");
```

+ 使用`read_line()`读取的字符串包含末尾的`/n`，因此需要首先`trim()`继而再`parse()`

### 处理无效输入

+ `expect`的行为是直接终止程序，另一种处理异常的方式是使用`match`表达式
