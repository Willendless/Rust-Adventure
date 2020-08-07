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

#### Floating-Point Types

#### Numberic Operations

#### The Boolean Type

#### The Character Type

### 复合类型

复合类型将多个值组合在一种类型中。rust有两种原始组合类型。

#### The Tuple Type

#### The Array Type


## 函数

## 注释

## 控制流




