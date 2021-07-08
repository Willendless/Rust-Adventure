# ch4 Understanding Ownership

+ Rust通过ownership在不需要垃圾收集器的情况下提供内存安全保证。
+ 所有权系统是一系列编译器在编译时需要保证的规则，rust基于这些规则管理内存。

## 1. 所有权规则

+ rust中每一个值都有其对应的*owner*.
+ 任意时刻仅能有一个*owner*.
+ 当*owner*超出作用域时，其对应的值会被dropped.

### 1.1. 内存的分配和释放：drop

对于存储在堆上的大小可变的变量，rust保证其内存在超出其作用域后被释放。这个特殊的函数被称为`drop`，由rust在闭合大括号处自动调用。

### 1.2. 变量和数据交互的方式：move和clone

#### 1.2.1. move（转移所有权）

```rust
let s1 = String::from("abc);
let s2 = s1;
```

编译器在作用域末尾调用drop()函数释放堆内存，为避免二次释放，保证在同一个作用域内不存在两个指针指向同一个对象实例。当一个对象变量赋给另一个时，视为前一个变量**move**到后一个变量（类似于特殊的shallow copy），同时前一个变量因为out of scope而无效。

#### 1.2.2. clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

若要deep copy可以使用`clone`方法。同时复制指针、元数据和指针指向的数据。

#### 1.2.3. Stack-Only Data： Copy

```rust
let x = 5;
let y = x;
```

+ 如果一个类型具有`Copy`trait，则旧变量在被赋值给新变量后仍能使用。
+ 任何实现了`Drop`trait的类型都不能具有`Copy`trait。
+ 总的来说所有scalar值都可以`Copy`，即stack-only data。

### 1.3. 所有权和函数

+ 根据函数传值原则，通过传值传递后，实参变量即out of scope。
+ 函数的返回值也能够转移所有权。
+ 因此可以通过传参将所有权传给函数，再通过返回值将所有权返回。

```rust
fn calc_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
```
## 2. References and borrowing

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1);
    // do something
}

fn calc_length(s: &String) -> usize {
    s.len()
}
```

+ java内存模型是只有简单容器变量，即所有heap创建的变量，都仅指针绑定。
+ 而rust内存模型中heap创建的变量则是复合变量，所绑定的内存中除了指针还包括其它变量。例如对于String类型来说，除了指向字符串的指针，还包括len,capacity,name等。
+ 因此所有java对象变量都是引用，而rust需要通过&操作符获取指向指针（即其它元数据）的指针。
+ `&s`创建一个指向`s`但不具有原变量`s`的引用，因此所有权不会被转移，因此不需要返回引用来返回所有权。通过引用传参被称为**borrowing**。

### 2.1. 可修改引用

#### 2.1.1. 规则

+ 首先，引用指向的变量需要是可修改的。
+ 其次，在一个确定的作用域内，对于一个确定的数据仅能有一个可变引用。

#### 2.1.2. 目的：在编译期阻止数据竞争

+ 类似于`race condition`，数据竞争发生的原因有以下三种情况：
    1. 两个或多个指针同一时刻访问同一个数据
    2. 至少一个对指针进行写操作
    3. 对数据的访问操作不施加同步机制
+ 同一数据同一作用域内数据访问规则：
    1. **可修改引用**至多只能有一个
    2. **不可修改引用**可以有多个
    3. 不可同时存在**可修改引用**和 **不可修改引用**

> *引用的作用域：*  
> a reference's scopte starts from where it is introduced and continues through the last time that reference is used.

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);
// 从改行之后r1和r2都不再被使用

let r3 = &mut s;
println!("{}", r3);
```

### 2.2. Dangling引用

rust编译器保证：

1. 不存在dangling引用，即引用总是有效的。
2. 引用指向的数据总是在引用的生命周期结束之后out of scope。

```rust
fn dangle() -> &String {
    let s = String::from("hello");

    &s // 返回一个指向s的引用
} // s超出作用域被drop，&s会指向无效内存，编译器报错
```

### 2.3. 总结：引用的使用规则

+ 任意时刻，保证只存在任意多个不可修改引用或一个可修改引用
+ 引用必须有效(即不可 dangling references)

## 3. 特殊的引用：slice

### 3.1. 字符串遍历

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

### 3.2. String slices

+ 创建语法：`&s[starting_index..ending_index]` (左闭右开)
    + 一些简写：
        1. `let slice = &s[..2]`
        2. `let slice = &s[3..]`
        3. `let slice = &s[..]`
+ slice类似于View视图的概念。通过在内存中存储一个起始引用和slice的长度实现。
+ **String slice的类型是&str**，是**不可修改引用**。

> String slice的范围索引必须在有效的UTF-8字节边界。

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..];
}

fn main() {
    let s = String::from("Hello world!");

    let first_word = first_word(&s[..]); // s is borrowed as immutable

    //s.clear();  compile error: cannot borrow 's' as mutable, 不能同时存在immutable和mutable引用

    prinln!("the first word is: {}", word); // 这里使用了immutable引用
}
```

#### 3.2.1. 字符串字面量作为slice

+ `let s = "Hello, world!"`
+ 这里s的类型是`&str`，因为`&str`是immutable引用因此字符串字面量不可修改

#### 3.2.2. 其它类型的slice

```rust
let a = [1, 2, 3, 4, 5, 6];
let slice = &a[1..];
```

对于i32类型数组的slice，类型是&[i32]。

## 4. 总结

+ ownership
+ borrowing
+ slice
+ 编译期保证rust程序的内存安全：在owner超出作用域时自动释放内存。
