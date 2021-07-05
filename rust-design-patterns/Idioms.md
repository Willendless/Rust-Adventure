# Idioms

## 1. Use borrowed types for arguments

> 使用deref coercion的目标。

应当使用*borrowed type*而非*borrowing the owned type*例如：使用`&str`而非`&String`，使用`&[T]` 而非`&Vec<T>`，使用`&T`而非`&Box<T>`

对于那些owned类型已经存在，使用*borrowed types*能够减少转换的次数。例如：`String`需要一层转换，因此`&String`需要两层转换。可以使用`&str`避免这种情况，即让`&String`强制转换成`&str`。

## 2. 使用`format!`连接字符串

对可变`String`可以使用`push`，`push_str`或者`+`。但是用`format!`更加方便，尤其是当混合有字面量和非字面量时。

但是使用一系列`push`通常效率更高，尤其是string已经被预先分配需要的内存。

## 3. Constructuors

使用静态方法`new`来创建对象

## 4. The `Default` Trait

rust无法抽象出"所有实现了`new()`方法的类型"。但是可以用`Default`trait替代。可被用于容器和其它泛型类型，例如`Option::unwrap_or_default()`

也可以使用`#[derive(Default)]`，只要所有域都实现了`Default`trait。

构造器能够有多个参数，也可以多种构造器。`default()`方法只能有一个，且没有参数。

## 5. Collections are smart pointers

使用`Deref`将集合视为智能指针使用，同时提供owning和borrowed视角。

```rust
use std::ops::Deref;

struct Vec<T> {
    data: T,
    // ...
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        //..
    }
}
```
