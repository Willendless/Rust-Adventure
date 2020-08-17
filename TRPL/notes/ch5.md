# ch5 Using Structs to Structure Related Data

oop中，*struct*类似于对象的数据属性。这一章讨论

+ tuple和struct的差异
+ 如何使用struct
+ 如何定义和struct关联的方法以明确struct数据的行为。

rust中，struct和enums是在程序范围内构建新类型的基本组件，并能很好的利用rust的编译期类型验证。

## 定义和实例化struct

### 和tuple的异同

+ 同：组合不同类型数据。
+ 异：struct可以命名每一个值，因此访问field的值时不依赖于field的顺序。

### 实例化struct

```rust
struct User {
	username: String, 
	email: String,
	sign_in_count: u64,
	active: bool,
}

let u1 = User {
	username: String::from("Hello"),
	email: String::from("123"),
	sign_in_count,
	active,
}

let u2 = User {
	..u1
}
```

>  不可单个域设置是否immutable，只能整个结构体变量设置

+ *field init shorthand syntax*: 当初始化的域和初始化变量具有相同命名时，可以省略初始化变量
+ struct update syntax: 使用另一个同类型变量的域进行初始化

### 其它类型的struct

rust中struct一共有三种：

1. c struct
2. tuple structs
	+ tuple struct的各个域不具有名字但是具有类型。
	+ tuple struct主要用于需要给予整个tuple一个名字并且让该tuple和其它tuple有所区分的情况。
	+ 使用：
		+ `struct Color(i32, i32, i32);`
		+ `struct Point(i32, i32, i32);`
		+ `let black = Color(0, 0, 0);`
		+ tuple struct的用法和tuple相同，索引访问+destructure。
	+ 注：即使各个域的类型相同，不同tuple struct的类型不同，例如上述的`Color`和`Point`类型。
3. unit-like structs
	+ 不含有域的结构体，类似于`()`

### 结构体数据的所有权

若要在结构体中存储属于其它变量引用，则需要设计*lifetimes*的概念。*lifetimes*保证了结构体拥有的数据引用始终有效。

## struct格式化输出

+ debug打印模式：需要在结构体头加上记号,`#[derive(Debug)]`
	+ `{:?}`: 同一行
	+ `{:#?}`: 多行

## methods

+ 使用impl块，方法的第一个参数固定为`&self`，且不需要类型。
	+ 方法同样可以
		1. take ownership of self (`self`)
		2. borrow self immutably (`&self`)
		3. borrow self mutably (`&mut self`)
+ rust具有自动解引用的能力，因此均可用点调用.
	+ `p1.distance(&p2)`和`(&p1).distance(&p2)`等价

## associated method

+ 在impl块内部，也可以定义不以`self`为参数的函数，这被称为associated method。
	+ 例如：`String::from`
	+ 通过类型名加上`::`调用
+ associated method可用于实现构造器。

> 每个struct类型可以有多个impl块。
