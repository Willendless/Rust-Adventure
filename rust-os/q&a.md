# questions

## 为什么使用trait而不是class，基于组合而不是继承的意义是什么。

继承自同一个类的多个子类可能具有不同的行为。

## lifetime和scope的区别是什么

scope通常指一对大括号包含的区域内部。lifetime则可能跨越多个函数体。

## reborrow是什么，有什么作用

```rust
let mut x = V(..); // x: write
let a = &mut x; // x: []; *a: write, a: read
let b = &(*a); // *a: read; {*b, b}: read
```
