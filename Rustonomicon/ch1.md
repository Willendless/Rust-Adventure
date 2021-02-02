ch1: Meet Safe and Unsafe

## 1. How Safe and Unsafe Interact

> Use `unsafe` to indicate the existence of unchecked contracts on functions and trait declarations.


声明为`unsafe`的代码块表明其内部的`unsafe`操作遵守对应的协议。例如，传给`slice::get_unchecked`的索引应当在slice范围之内。

声明为`unsafe`的trait的实现表明该实现遵守trait的协议。例如，实现`Send`的类型在被move给另一个线程时应当是安全的。

标准库中一些unsafe函数如下：

+ `slice::get_unchecked`执行无检查的索引
+ `mem::transmute`将对应值解释为另一个种类型
+ 每一个指向sized类型的原始指针有一个`offset`方法会在传入的偏移超出边界时导致UB
+ 所有FFI函数都是`unsafe`调用，因为其它语言会执行rust编译期无法检查的任意操作

标准库同时定义了下列unsafe的trait

+ `Send`是一个marker trait(没有API的trait)，承诺实现者实例在线程之间的move是安全的。
+ `Sync`是一个marker trait，承诺线程之间能够安全地通过共享引用共享实现者。
+ `GlobalAlloc`允许为整个程序定义内存分配器。

对于safe rust，其保证如下根本属性：

> **No matter what, Safe Rust can't cause Undefined Behavior.**

rust这种safe/unsafe相互区分的设计意味着safe和unsafe rust之间存在一种不对称的信任关系。safe rust必须信任unsafe rust，而unsafe rust无法信任safe rust。

例如，`BTreeMap`要求键实现`Ord`trait（全序关系）。然而，`BTreeMap`包含unsafe的代码。这意味着unsafe代码需要尽可能鲁棒以处理`Ord`的错误实现（即不满足全序关系的实现）。

`unsafe`代码不能简单地认为safe rust代码都被正确实现。因为`BTreeMap`的键类型是泛型，信任`Ord`意味着信任`Ord`的任意实现。这种风险是非常高的。

解决无限泛型信任这个问题本是`unsafe`trait的任务。例如，`BTreeMap`理论上本可以要求其内的键实现下面的trait

```rust
use std::cmp::Ordering;
unsafe trait UnsafeOrd {
    fn cmp(&self, other: &self) -> Ordering;
}
```

此时一个类型需要使用`unsafe`以实现`UnsafeOrd`，这表明该实现满足了所有`UnsafeOrd`trait要求的协议。这样的话，`BTreeMap`内部的`unsafe`实现就可以将键的实现视为正确的。如果存在问题，则可以认为unsafe trait的实现存在问题。

是否将trait标记为`unsafe`是API的设计选择。然而rust应当尽力减少`unsafe`的使用。`Send`和`Sync`被标识为unsafe是因为线程安全性是一个底层特性，unsafe代码不太可能像`BTreeMap`检查Ord的错误实现那样检查它们。

编写`unsafe`trait时应当遵循相同的原则。如果`unsafe`代码无法合理地检查trait的错误实现，则应当使用`unsafe`trait。

同时需要注意到，当类型的各个域都实现了`Send`和`Sync`时，则该类型被自动实现了`Send`和`Sync`。

safe和unsafe rust代码的这种区分是为了尽可能让safe rust易于编写，但是在写unsafe rust代码时需要更加小心。


## 2. What Unsafe Can Do

unsafe rust和safe rust唯五的区别如下

+ 解引用原始指针
+ 调用`unsafe`代码（包括c函数，编译器指令和原始分配器）
+ 实现`unsafe`traits
+ 可变静态变量
+ 访问`union`的域

这些操作的错误使用会导致未定义行为。

和C不同，rust中的未定义行为非常有限。核心语言只为了避免下列情况的出现

+ Dereferencing (using the * operator on) dangling or unaligned pointers (see below)
+ Breaking the pointer aliasing rules
+ Calling a function with the wrong call ABI or unwinding from a function with the wrong unwind ABI.
+ Causing a data race
+ Executing code compiled with target features that the current thread of execution does not support
+ Producing invalid values (either alone or as a field of a compound type such as enum/struct/array/tuple):
  + a bool that isn't 0 or 1
  + an enum with an invalid discriminant
  + a null fn pointer
  + a char outside the ranges [0x0, 0xD7FF] and [0xE000, 0x10FFFF]
  + a ! (all values are invalid for this type)
  + an integer (i*/u*), floating point value (f*), or raw pointer read from uninitialized memory, or uninitialized memory in a str.
  + a reference/Box that is dangling, unaligned, or points to an invalid value.
  + a wide reference, Box, or raw pointer that has invalid metadata:
    + dyn Trait metadata is invalid if it is not a pointer to a vtable for Trait that matches the actual dynamic trait the pointer or reference points to
    + slice metadata is invalid if the length is not a valid usize (i.e., it must not be read from uninitialized memory)
  + a type with custom invalid values that is one of those values, such as a NonNull that is null. (Requesting custom invalid values is an unstable feature, but some stable libstd types, like NonNull, make use of it.)


虽然存在其它有问题的操作，rust仍认为下面的操作是安全的。

+ 死锁
+ [竞态条件](https://doc.rust-lang.org/nomicon/races.html)
+ 内存泄露
+ 调用析构函数失败
+ 整型溢出
+ 程序终止(abort)
+ 删除产品数据库


## 3. Working with Unsafe

有函数如下

```rust
fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx < arr.len() {
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}
```

注意到有可能它会被错误实现为

```rust
fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx <= arr.len() {
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}
```

可以注意到，虽然我们改变的是safe代码，但仍然可能导致未定义行为。这就是safety的根本问题：**非本地性（non-local）**。unsafe代码仍需要依赖于"safe"操作建立的状态。

我们说safety是模块化的，因为是否进入非安全代码块，并不受其他部分代码正确性的影响。例如，i）对一个slice进行的是否是不安全索引，ii）对切片进行unchecked索引时，无需担心切片是否为null或者包含了未初始化的内存。然而，safety也是非模块化的，因为程序本质上是有状态的，unsafe操作可能依赖于任意其它的状态。

当考虑实际的状态时，这种非本地性会更加复杂。如下

```rust
use std::ptr;

// Note: This definition is naive. See the chapter on implementing Vec.
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types.
// See the chapter on implementing Vec.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // not important for this example
            self.reallocate();
        }
        unsafe {
            ptr::write(self.ptr.offset(self.len as isize), elem);
            self.len += 1;
        }
    }
}
```

上面的代码是合理的。然而如果加上下面的方法

```rust
fn make_room(&mut self) {
    // grow the capacity
    self.cap += 1;
}
```

此时，虽然增加的是safe的代码。但是改变`cap`违反了`Vec`的`cap`域需要满足的不变式（反映`Vec`实例被分配的空间）。因此，`push`中的`unsafe`代码污染了整个模块。通常唯一的解决办法是限制`unsafe`代码的范围不能超过该模块的边界。

然而，上述代码能够正常工作。`make_room`的存在并没有问题。因为该方法不是`pub`的，因此只有定义了该方法的模块能够调用它。同时`make_room`直接访问了`Vec`的私有域，因此它只能被写在Vec的相同域内。

因此，依赖于复杂不变式写出完全安全的抽象是可能的。

## 4. 小结

unsafe代码必须信任某些safe代码，但是不应该信任泛型safe代码。类似的，私有成员的限制对非安全代码也很重要，它阻止我们无条件信任所有safe代码。
