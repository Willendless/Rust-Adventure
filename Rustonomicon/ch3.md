# 1. ch3 Ownership and Lifetimes

## 1.1. References

rust中存在两种引用：

+ shared引用：`&`
+ mutable引用：`&mut`

引用遵循下列规则：

+ 引用不能outlive其指向的对象
+ 可变引用无法被别名化(aliased)

## 1.2. Aliasing

在单线程，无中断执行的语境下，aliasing的定义是：*variables and pointers alias if they refer to overlapping regions of memory.*

“可变引用无法被别名化”这条规则使得更多的优化成为了可能。

## 1.3. lifetimes

rust通过*lifetimes*保证这些规则。

Def: *Lifetimes are named regions of code that a reference must be valid for.*

+ 每一个 `let` 语句隐含地引入了一个scope

例如下面的代码

```rust
let x = 0;
let y = &x;
let z = &y;

'a: {
    let x: i32 = 0;
    'b: {
        let y: &'b i32 = &'b x;
        'c: {
            let z: &'c &'b i32 = &'c y;
        }
    }
}
```

当把引用传到外部scope中会让rust推导出一个更大的lifetime

```rust
let x = 0;
let z;
let y = &x;
z = y;

'a: {
    let x: i32 = 0;
    'b: {
        let z: &i32 = 0;
        'c: {
            // 这里必须使用'b，因为引用y会被
            // 传到外部域
            let y: &'b i32 = &'b x;
            z = y;
        }
    }
}
```

### 1.3.1. ex: references that outlive referents

```rust
fn as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data);
        return &'a s
    }
}

fn main() {
    'c: {
        let x: u32 = 0;
        'd: {
            // An anonymous scope is introduced because the borrow does not
            // need to last for the whole scope x is valid for. The return
            // of as_str must find a str somewhere before this function
            // call. Obviously not happening.
            println!("{}", as_str::<'d>(&'d x));
        }
    }
```

正确的写法是

```rust
fn to_string(data: &u32) -> String {
    format!("{}", data)
}
```

为了能够返回`&'a str`，唯一的方式是将它作为`&'a u32`的一个域，显然这种情况无法发生。

### 1.3.2. ex: aliasing a mutable reference

```rust
let mut data = vec![1, 2, 3];
let x = &data[0]; // x是共享引用
data.push(4); // 对data取可变引用
println!("{}", x);

'a: {
    let mut data: Vec<i32> = vec![1, 2, 3];
    'b: {
        // 'b is as big as we need this borrow to be
        // (just need to get to `println!`)
        let x: &'b i32 = Index::index::<'b>(&'b data, 0);
        'c: {
            // Temporary scope because we don't need the
            // &mut to last any longer.
            Vec::push(&'c mut data, 4);
        }
        println!("{}", x);
    }
}
```

当使用`push`方法对`data`取可变引用时同时存在活跃的共享引用`x`，这违反了第二条规则。

但是对于borrow checker来说，其并不知道x是对`data`的子路径的引用，也不理解`Vec`。它只知道`x`必须在`'b`内存活以被打印。之后的`Index::index`标记要求对`data`的引用在`'b`内也需要存活。调用`push`时，表明我们尝试创建`&'c mut data`。rust知道`'c`被包含在`'b`内，因为`&'b data`需要存活，所以报错。

+ 因为'c被包含在'b内，且`&'b data`在c之后仍需要有效，因此被拒绝。

### 1.3.3. the area covered by a lifetime

The lifetime (sometimes called a *borrow*) is *alive* from the places it is creates to its last use. The borrowed thing needs to outlive only borrows that are alive.

下列代码能够编译，因为在打印出`x`之后，不再需要它，因此其之后是dangling或是aliased并不会影响结果。

```rust
let mut data = vec![1, 2, 3];
let x = &data[0];
println!("{}", x);
data.push(4);
```

然而，如果该值有destructor。因为destructor在scope结尾被调用且被认为是对值得使用。因此无法成功编译。

```rust
#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}

let mut data = vec![1, 2, 3];
let x = X(&data[0]);
println!("{:?}", x);
data.push(4); // immutable ref to data still alive
// Here, the destructor for x is run and therefore this'll fail to compile.
// x.drop();
```

## 1.4. Limits of Lifetimes

对于下面无法编译的代码

```rust
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self { &*self }
    fn share(&self) {}
}

fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share();
    println!("{:?}", loan);
}
```

如下

```rust
struct Foo;

impl Foo {
    fn mutate_and_share<'a>(&'a mut self) -> &'a Self { &'a *self }
    fn share<'a>(&'a self) {}
}

fn main() {
    'b: {
        let mut foo: Foo = Foo;
        'c: {
            let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
            'd: {
                Foo::share::<'d>(&'d foo);
            }
            println!("{:?}", loan);
        }
    }
}
```

rust的生命周期系统根据`loan`的生命周期和`mutate_and_share`的函数签名，扩展`&mut foo`拥有生命周期`'c`。因此当其发现调用`share`时就会报错。

根据引用的语义，该例子的程序是正确的。但是由于生命周期系统过于粗糙，因此无法处理这种情况。这是因为rust无法理解`mutate_and_share`的borrow不会再被需要，因此保守得赋予其整个scope。

## 1.5. Lifetime Elision

某些情况下，函数签名中的生命周期标注能够被省略。

*lifetime postion*是类型中能够标注生命周期的任意位置：

```rust
&'a T
&'a mut T
T<'a>
```

对于函数定义，*lifetime position*可以作为"input"或者"output"出现，elision规则遵守下列规则：

+ 每一个input位置的elided lifetime做为一个独立的lifetime参数
+ 如果只有一个input lifetime position，则该声明周期被赋给所有elided output lifetime
+ 如果存在多个input lifetime position，但是其中之一是`&self`或者`&mut self`，则`self`的生命周期被赋给所有elided output lifetimes。

```rust
fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command                  // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded

fn new(buf: &mut [u8]) -> BufWriter;                    // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // expanded
```

## 1.6. Unbounded Lifetimes

unsafe代码经常会凭空生成引用或者生命周期（例如：对原始指针进行解引用操作）。通过这种方式生成的引用的生命周期是无界(*unbounded*)的。大多数情况下，这种生命周期可以被视为`'static`，但是它实际上比设为`'static`更加强大，例如`&'static &'a T`无法通过类型检查但是无界生命周期能够适配为`&'a &'a T`。

事实上几乎不存在`'static`的引用。`transmute`和`transmute_copy`是两种主要的例外。我们应当倾向于尽早确定无界生命周期的边界，尤其是当该引用跨越函数边界的时候。

给定一个函数，任何无法从输入推得其生命周期的输出引用都是无界的。例如：

```rust
fn get_str<'a>() -> &'a str;
```

上述函数生成的`&str`具有无界生命周期。为了避免无界生命周期，可以在函数边界使用生命周期省略。如果输出生命周期被省略，则它一定被某个输入生命周期限定。

然而在函数内部，限定生命周期很容易出错。限定生命周期最安全也是最容易的方式是让它从某个具有有界生命周期的函数处返回。
