# ownership and lifetime 1

## ownership和copy的语义

```rust
let x = String::from("Helo!");
let y = x.clone();
```

```	rust
fn main() {
 push    rbx
 sub     rsp, 48 // 局部变量x和y的空间
 mov     rbx, rsp // x变量放在rbx寄存器
 let mut x = String::from("Hello!");
 mov     rdi, rbx // x变量的值放入rdi寄存器作为第一参数传参
 call    <alloc::string::String as core::convert::From<&str>>::from
 lea     rdi, [rsp, +, 24] // rdi寄存器增加24，即y变量起始地址
 let y = x.clone();
 mov     rsi, rbx // x变量起始地址放入rsi寄存器作为第二个参数传参
 call    qword, ptr, [rip, +, _ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hb9c52a0335097fe4E@GOTPCREL] // 调用clone函数，第一个参数为y，第二个参数为x
 lea     rdi, [rsp, +, 24] // rdi重新赋值为y
 }
 call    core::ptr::real_drop_in_place // 释放y堆内存
 mov     rdi, rsp // rdi赋值为x
 call    core::ptr::real_drop_in_place // 释放x堆内存
 }
 add     rsp, 48 // 栈复位
 pop     rbx // rbx弹出
```
