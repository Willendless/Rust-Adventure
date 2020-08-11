# ch3 Common Programming Concepts

## Variables and Mutability

+ 变量默认immutable，除非加上mut关键字
+ 对于较大的数据结构，使用mut较好。对于较小的数据结构，使用不可变变量，并在需要修改时创建新的实例较好。

### 变量和常量的差异

*语法* `const MAX_POINTS: u32 = 100_000;`

1. 常量不能以mut修饰
2. 使用`const`声明常量而非`let`，并且常量的类型必须被注明
3. 常量能够在任何scope内声明，包含global scope。
4. 常量仅能够被常量表达式设置，而不能是函数返回值或其它在运行时计算的值。

*常量命名惯例*: 全部使用大写，单词间用`_`分隔，数值常量中可以插入`_`以提高可读性。

整个程序运行期间，常量在其声明的scope中均有效。

### Shadowing

使用同样的名字声明变量，被成为shadowed by the second。

#### shadowing和mut的差异

1. shadowing保证了变量immutable
2. shadowing能够改变变量类型，而无需创造新的变量名

## 数据类型

rust是statically typed language，这意味着编译期需要知道所有变量的类型。

编译器通常能够基于变量值以及使用推断出变量类型，当编译器无法推断时需要指明变量类型。

`let guess : u32 = "42".parse().expect("Not a number!")`

### 标量类型

标量表示单个值。rust标量有4种可能类型。

#### Integer

|Length|Signed|Unsigned|
|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

> `isize/usize`取决于架构，64位机表示64位，32位机表示32位。

##### 整型字面量

|Number literals|Example|
|---|---|
|Decimal|98_222|
|Hex|0xff|
|Octal|0o77|
|Binary|0b1111_0000|
|Byte(`u8`only)|b'A'|

+ 整型的默认类型是i32
+ isize/usize通常用于索引一些集合
+ 整数溢出：在debug模式下编译后，rust会加入整数溢出检查。runtime发生整型溢出时，程序会报panic。而release模式编译(使用`--release`参数)时，程序不会报panic。

#### Floating-Point Types

rust支持两种浮点类型，f32和f64。默认情况下(浮点数字面量)，浮点类型是f64。

#### The Boolean Type

`bool`：`true`和`false`

#### The Character Type

`char`：rust的char类型用于表示字符，大小为4字节，用于表示unicode标量值。char类型字面量由单引号包围。

### 复合类型

复合类型能够将多个值组合在一种类型中。rust有两种原始组合类型：tuples和arrays。

#### The Tuple Type

Tuple是将多个不同类型的值组合在一起的通用方式。Tuples有固定的长度，一旦被声明，tuple的大小就不能改变。

##### tuple使用示例

```rust
func main() {
    let tup: (i32, f64, u8) = (500, 6.4, 8);
    // user pattern matching to *destructure* a tuple value
    let (x, y, z) = tup;
    println!("The value of y is : {}", y);
    let a = tup.0
}
```

##### tuple 操作

+ *destructure*: tuple的解包
    + `let (x, y, z) = tup;`
+ element access: 使用`.`运算符加上索引的值
    + `let a = tup.0`

#### The Array Type

+ 和tuple不同，array的每一个元素需要是相同类型。
+ 和其它编程语言不同，rust的数组长度是固定的。

##### Array使用示例

声明类型时需要同时指明元素*类型*和数组*长度*。

```rust
func main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]
    // 包含5个相同元素的数组的创建
    let a = [3; 5]
    let a = [5, 5, 5, 5, 5]
    // 获取数组元素
    let first = a[0]
    // 获取数组长度
    let len = a.len()
}
```
+ array通常用于需要在stack中分配数据的情况
+ array通常用于需要表示固定数量的元素
+ array越界访问：程序产生运行时错误并错误退出

## 函数

+ 函数命名惯例：snake case，所有字母小写，单词间用'_'分隔
+ 函数定义由`fn`开始
+ 函数参数必须指明类型

### 语句和表达式

rust是expression-based语言，这意味着语句和表达式有严格的区分。

+ statements执行一些操作但不会返回值。
    + `let y = 6;`
    + 函数定义也是语句
    + 语句不会返回值，因此不能将let语句赋值给另一个变量，例如`let x = (let y = 5);`
+ expressions求得结果值，并能够作为语句的一部分。
    + `let y = 6;`中6是一个表达式
    + 调用函数和宏均是表达式

```rust
let y = {
    let x = 3;
    x + 1
}
```

由`{}`构成的构建新scope的代码块也是表达式，上面例子中该代码块求值得到4。表达式没有结尾的`;`。若在该示例结尾加上`;`则该表达式将被转换位语句。

> 代码块表达式求值结果等于该代码块中最后一个表达式的结果值。

### 带返回值的函数

+ rust函数的返回值类型由`fn func() -> i32 {}`给出
+ rust中函数的返回值等价于函数体最后一个表达式的结果。

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## 控制流

### if表达式

+ 表达式条件的类型必须是bool类型。并且rust不会将非bool类型自动转换为bool类型。

```rust
if ok {

} else if {

} else {

}
```

#### 在语句中使用if表达式

+ 由于if是表达式，因此能够用于赋值语句
+ 注意，此时if表达式的不同arm的结果需要是同一类型，这样编译器在编译期就能知道变量类型。而非在runtime。

```rust
let number = if condition { 5 } else { 6 };
```

### 循环

rust有三种循环：loop，while和for。

#### loop循环

除非明确终止循环(通过ctrl+c或者`break`)，`loop`关键字无条件重复执行代码块。

loop的一种用例是重复一个可能会失败的操作例如验证线程是否结束。当需要获取该操作的返回值时，可以在break表达式后添加该值以将该值传递到loop循环外部，

```rust
fn main() {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
}
```

#### while循环

```rust
while condition {

}
```

#### for循环

for循环用于遍历集合元素。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // do something
    }

    for number in (1..4).rev() {
        // do something
    }
}
```

+ Range类型由标准库提供，用于生成一个整数序列。
