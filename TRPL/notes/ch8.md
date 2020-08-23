# ch8 Common Collections

集合和内置的数组以及元组的区别在于集合指向的数据存储在堆上。这意味着集合可以存储可变数量的数据，并且可以在运行时添加或删除。本章介绍*vector, string, hashmap*三种集合

## storing lists of values with vectors

+ 相同类型或者利用enum存储不同类型
+ 堆中存储在相邻位置

### 创建vector

```rust
// 可以声明所存储元素的类型。但是也可以不使用，rust可以根据插入元素的类型推测。
let v: Vec<i32> = Vec::new(); 
// 创建具有初始值的vec更常见。（i32类型）
let v = vec![1, 2, 3];
```

### 更新vector

需要加上`mut`关键字，才能修改vec。

```rust
let mut v = Vec::new();
v.push(3);
v.pop();
```

### drop vector

当vector在超出作用域被释放时，其内所有元素均被drop掉。

```rust
let v = vec![1, 2, 3, 4];
```

### 读取vector元素

```rust
// 两种方法
let v = vec![1, 2, 3, 4, 5];
// 返回类型：&i32, 若out of range会panic
let third: &i32 = &v[2];
// 返回类型：Option(&T)
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => // do something else 
}
```

+ 当引用有效时，borrow checker施加所有权和borrowing规则保证该引用和其它对vec内部元素的引用有效。
+ 注：vec是可修改引用，而不可同时存在immutable和mutable引用。

### 迭代vector内值

```rust
let v = vec![100, 32, 27];
for i in &v {
    // do something
}
for i in &mut v {
    *i += 50;
}
```

### 利用enum在vec中存储多个类型的值

```rust
enum A {
    Int(i32),
    Float(f64),
}
let row = vec![
    A::Int(3),
    A::Float(10.12),
];
```

+ 利用`enum`配合`match`语句能够避免对vec内的元素执行错误操作。
+ 另一种方式是使用trait。

## string

rust核心语言中，仅有字符串切片`str`一种类型，通常以borrowed形式即`&str`类型出现，UTF-8编码。例如，字符串字面量存储在程序的二进制文件中，因此是字符串切片。

rust标准库提供了多种字符串类型。其中`String`是可增长、可修改，UTF-8编码的字符串类型。其它还包括`OsString`，`OsStr`，`CString`，`CStr`。

### 创建字符串

```rust
// 1. String::new()
let mut s = String::new();
// 2. "".to_string()
let s = "hello world".to_string();
// 3. String::from()
let s =  String::from("hello world");
```
### 更新字符串

#### 1. append字符串

```rust
let mut s = String::from("foo");
// 1. push_str("") 连接一个字符串
let s2 = "abc";
// s.push_str(s2); s2的所有权被转移，因此通常直接使用字面量
s.push_str("abc");
// 2. push('') 连接一个字符
s.push('l');
```

#### 2. concatenate字符串

##### 利用`+`

```rust
let s1 = String::from("Hello ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

+ `+`操作符：调用`fn add(self, s: &str) -> String`方法（注：标准库中利用的是泛型）。
+ 因此`+`后，`s1`无效所有权被转移，而s2仍有效。
+ 编译器能够利用*deref coercion*将`&s2`转换成`&s2[..]`，因此编译不会报错。

##### 利用`format!`宏

```rust
let s1 = String::from("a");
let s2 = String::from("b");
let s3 = String::from("c");

let s = format!("{}-{}-{}", s1, s2, s3);
```

+ `format!`宏不会转移参数所有权

### 索引字符串

rust的`String`不支持索引。

#### `String`内部表示

`String`是`Vec<u8>`类型的包装类。实际存储的是utf-8编码的字节，因此若直接使用索引无法确定字母边界。编译器在编译期阻止该行为。

#### `String`的三种视角: 因此索引的意义不明确

1. bytes
2. unicode scalar values - (chars每个占用两字节)
3. letters(可能多个unicode scalar values组成一个letter)

### 切分字符串

rust能够使用`[]`以及一个范围创建包含确定字节的字符串切片。

```rust
let hello = "12113阿斯蒂芬";
let s = &hello[0..4]; // 字符串的前4个字节
// let s = &hello[0..1]; // 运行时 panic
```

### 迭代字符串

```rust
// 迭代chars
for c in "asdf".chars() {
}
// 迭代bytes
for b in "asdf".bytes() {
}
```

## Storing keys with Associated Values in Hash Maps

### 新建HashMap<K, V>

#### 1. 利用New()方法

```rust
use std::collections::HashMap;

let mut scores = HashMap::New();
scores.insert(String::from("a"), 10);
```

+ `HashMap`不存在与prelude中。
+ 和vector类似，`HashMap`中`Key`需要时相同类型，`Value`类似。

#### 2. 利用迭代器和tuples向量的`collect`方法

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> = 
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

+ `collect`能够将tuple向量转换成多种数据结构，因此需要指明是HashMap。
+ `HashMap<_, _>`表示rust能够推测其内的数据类型。

### HashMap和entry的所有权 

1. 对于实现了`Copy`trait的类型，其值被copy进hash map。
2. 对于owned values，其所有权会被move并且hashmap成为其拥有者。
3. 对于引用值，rust保证引用指向的值在hash map有效时始终有效。

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name和field_value此时都无效
```

### 访问HashMap中的值

#### get方法

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("a"), 10);
scores.insert(String::from("b"), 50);

let k = String::from("a");
let v = scores.get(&k);
```

+ `v`的类型是`Option<&v>`，因此需要处理`Option`。

#### 迭代HashMap中的键值对

+ 和向量类型

```rust
for (key, value) in &scores {
    // do something here
}
```

### 更新HashMap

+ 通过`insert`方法能够overwrite一个旧值

#### 1. 当HashMap中没有对应元素时插入值

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("a"), 10);
scores.entry(String::from("a")).or_insert(50);
scores.entry(String::from("b")).or_insert(50);
println!("{:?}", scores);
```

+ `entry`方法: 以键作为参数，返回表示一个值是否存在的`Entry`类型
+ `or_insert`方法: 若Entry表示值存在，则返回key对应的值的可变引用；否则，将参数作为新的值插入并且返回新值的可变引用。

#### 2. 基于旧值更新

`entry`方法返回可变引用`&mut V`，因此可以通过其返回值进行更新操作。

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}// count invalid after this point
```

### Hashing Functions

HashMap默认使用"cryptographically strong"hashing函数并且能抗DoS攻击。同时可以通过指明一个不同的*hasher*，使用另一个hash函数。*hasher*是一个实现了`BuildHasher`trait的类型。
