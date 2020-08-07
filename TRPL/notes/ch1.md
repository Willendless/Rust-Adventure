# ch1 Getting Started

## hello world

### 程序解释

+ 函数体由大括号包围，第一个大括号的风格应当为和函数名同级
+ `println!`是rust宏而非函数，这可以通过`!`发现
+ 语句以`;`号结尾

### 编译和运行

rust程序需要先编译后运行。使用`rustc`编译，在unix系统下生成`main`可执行文件，在wins下生成`main.exe`和`main.pdb`文件，pdb文件包含了debug信息。

*ahead-of-time compiled* language: 可执行文件可以在不安装rust的情况下运行。

## hello cargo

+ 创建新目录：`cargo new hello_cargo`
  + cargo new会自动创建git仓库
+ rust中包被称为`crate`
+ build项目：`cargo build`，创建target/debug文件夹
  + `cargo build --release`，创建target/release文件夹，能够优化代码但是需要更长时间编译
    + 注：跑benchmark时请使用release参数
+ run项目：`cargo run`
+ check是否能通过编译：`cargo check`
  + 开发程序过程中，周期性check项目是一个好习惯

*convention*:

```shell
git clone someurl.com/someproject
cd someproject
cargo build
```


