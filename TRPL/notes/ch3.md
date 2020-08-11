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
    let first = a[0]
}
```
+ array通常用于需要在stack中分配数据的情况
+ array通常用于需要表示固定数量的元素
+ array越界访问：程序产生运行时错误并错误退出

## 函数

## 注释

## 控制流
