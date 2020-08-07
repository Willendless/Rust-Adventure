# ch5

## 内置数据类型

1. tuple: 内部可以是不同类型, 支持destructure
	+ `let tup: (i32, u64, u32) = (500, 6, 4);`
2. struct: 和tuple类似，内部需要命名
3. array: 定长数组，内部类型相同

## struct

+ 不可单个成员设置可修改性，只能整体设置
+ field init shorthand syntax: 域初始化缩写语法，同名的域和初始化变量简写
+ struct update syntax: 使用另一个同类型变量域的值初始化

1 tuple structs: unamed fields with type
	+ `struct Color(i32, i32, i32))`
2 unit-like structs: 和`()`行为一致
	+ 没有域的结构体

## ownership

*lifetimes*

## print

debug打印模式：需要在结构体头加上记号,	`#[derive(Debug)]`
	+ `{:?}`: 同一行
	+ `{:#?}`: 多行

## methods

使用impl块，方法的第一个参数固定为`&self`.rust具有自动解引用的能力，因此均可用点调用.
`p1.distance(&p2)`和`(&p1).distance(&p2)`等价

## associated method

初始化方法，用`::`语法

## 

直接将heap复合变量传参通常是为了避免调用者继续使用该实参。

