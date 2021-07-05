# thinking of rust

## RAII: Resource Acquisition Is Initialization

+ compiler handle hindsight cases like exception by placing destructors

### scoping rules and ownership

变量在其生命周期内拥有其绑定值的所有权，变量被释放时，需要释放所拥有的资源。

### scoping rules and lifetime

lifetime是对scoping的扩展，并不局限在本地scope，生命周期可能涵盖多个函数。

## Substructure Type System

+ 强类型系统
+ 线性类型系统
+ 共享引用和可变引用
+ borrow和reborrow
