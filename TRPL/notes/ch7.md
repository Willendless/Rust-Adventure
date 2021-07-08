# ch7 Managing Growing Projects with Packages, Crates and Modules

总的来说，rust的模块系统包含以下几个特性：

1. Packages：由cargo提供，使得crates能够build、test和share。
    + 一个package可以包含多个binary crate以及可选的一个library crate。
    + 随着项目代码的增长，可以将部分代码提取到独立的crate构成外部依赖。
2. Crates: modules的树形组织，有library和executable两种。
3. Modules和use: 为代码组织、作用域和路径的私有性提供控制能力。
4. Paths：命名结构体、函数、变量或者module的方式。

## 1. Packages and Crates

### 1.1. Package

package由一个或多个crate组成，其内包含了一个*Cargo.toml*文件描述如何build各种crate。

+ package必须包含0个或1个library crate
+ package可以包含任意个binary crate
+ package必须包含至少一个crate

### 1.2. crate

crate分成binary和library两类。每个crate包含一个*crate root*文件，该文件是rustc编译的起始点。

+ *src/main.rs*: 是**和package同名的binary crate**的*crate root*。
+ *src/lib.rs*: 是和package同名的library crate的*crate root*。
+ 向*src/bin*目录中添加的每一个文件都是一个独立的binary crate。

> 通过`cargo new --lib [package name]`初始化library crate。

## 2. 定义模块以控制作用域与可见性

+ 模块被用于crate内部，crate类似于根目录，而module类似于子目录。
+ 模块由`mod`关键字起始，由`{}`包围，且可以嵌套。

## 3. Paths for referring to an Item in the Module Tree

### 3.1. path与module

+ path用于定位实体，一个path可能有两种形式：
    + *绝对路径*：使用crate name或字面量`crate`从crate root开始。
    + *相对路径*：使用`self`，`super`或当前模块标识符的从当前模块开始。
    + 绝对路径和相对路径中的一个或多个标识符由`::`区分。
    + 选择绝对路径或者相对路径取决于使用模块的代码和模块代码的耦合程度。通常使用绝对路径区分代码定义和实体调用更好。

### 3.2. module与privacy boundary

+ 模块定义了rust的*privacy boundary*: 保证内部实现细节外部不可见的界限。
+ privacy规则
    1. **default private**：应用到所有functions，methods，structs，enums，**modules**和constants实体。
    2. 子模块可见父模块的实现。
    3. siblings之间可见。
+ `super`关键字类似于`..`，引用上一级mod。

```rust
mod front_of_house {
    pub mod hosting { // 模块名可见
        // 函数实现可见
        pub fn add_to_waitlist() {
            super.serve();
        } 
        fn serve() {}
    }
    fn serve() {}
}

// eat_at_restaurant()和front_of_house定义在同一个module内，因此是siblings，所以可以互相访问。
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

注：**模块的pub关键字仅能够使得祖先模块引用当前模块名**，而不能引用当前模块内部的实现。除非内部实现也使用pub关键字。

### 3.3. Making Structs and Enums Public

+ 在struct前的`pub`仅能使结构体公开，但其内部的每一个域仍然是私有的，需要单独为每一个域添加`pub`。
+ 在enums前的`pub`使其内部所有变种公开，**enum的变种默认是公开的**。

## 4. 使用`use`将paths引入作用域

+ `use`关键字类似于**符号链接**，用于将**module或其他item**引入当前作用域。
+ **惯例**：
    1. **导入函数时**: 通常将函数的父亲module导入当前作用域。
    2. **导入structs，enums以及其它实体时**: 通常将整个路径导入当前作用域。
    3. **导入同名实体时**
        + 通常导入其父亲module。
        + 也可以使用别名，例如`use std::io::Result as IoResult`。
+ 使用`use`引入的item对于当前作用域而言可见，而对于当前作用域之外的代码而言是private的。为在外部作用域得以引用该符号链接，需要结合使用`pub`和`use`，这个技术被称为*re-exporting*

### 4.1. 使用外部包

需要使用外部packages时，需要在*Cargo.toml*中`[dependencies]`中添加。同时，在当前作用域使用`use`引入。

> std也属于外部crate，但是其和rust语言同时发布，因此不需要修改*Cargo.toml*，但是仍需要使用`use`。

### 4.2. `use`的简写

1. 当需要引入同一个模块或包的多个实体可以使用嵌套paths。
2. 通常在编写测试程序时，可以使用星号通配符将整个crate的`pub`实体引入当前作用域。

```rust
use std::io;
use std::io::write;
use std::{self, write}; // 嵌套paths
use std::collections::*; // glob operator
```

## 5. 区分不同文件中的modules

module类似于子目录，除了仅包含在crate root中，也可以独立地提取出来放在独立的文件中。此时原来的文件需要通过`mod [module name];`首先声明该module的存在。

此时若将模块`front_of_house`中的代码移入`src/front_of_house.rs`中，则`src/lib.rs`中的代码需要改为

```rust
mod front_of_house; // 分号结尾表示声明。这句话会让rust将front_of_house模块的内容导入当前文件
pub use crate::front_of_house::hosting; // 上一句导入之后，再可以引入对应实体
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

而`front_of_house.rs`文件对应为

```rust
pub mod hosting {
    pub fn add_to_waitlist();
}
```

此外也可以将`front_of_house`视为子目录，而将`hosting`中的代码放入`src/front_of_house/hosting.rs`文件中。此时，`src/lib.rs`中为

```rust
mod hosting; // rust需要从src/front_of_house/hosting.rs中引入hosting模块
pub use crate::front_of_house::hosting;
```

`src/front_of_house.rs`中代码为

```rust
pub mod hosting;
```

而`src/front_of_house/hosting.rs`中的代码为

```rust
pub fn add_to_waitlist() {}
```

## 6. 总结

rust模块系统中，package由crate组成，crate由模块组成。不同模块之间可以利用`use`语句通过paths引用。模块默认是private的。
