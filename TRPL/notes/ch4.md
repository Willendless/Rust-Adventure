# ch4

## 表达式和语句

+ block作为表达式

## move、clone和copy

编译器在作用域末尾调用drop()函数释放堆内存，为避免二次释放，保证在同一个作用域内不存在两个指针指向同一个对象实例。当一个对象变量赋给另一个时，视为前一个变量move到后一个变量，同时前一个变量因为out of scope而无效。

## References and borrowing

java内存模型是只有简单容器变量，即所有heap创建的变量，都和一个指针绑定。而rust内存模型中heap创建的变量则是复合变量，所绑定的内存中除了指针还包括其它变量。例如对于String类型来说，除了指向字符串的指针，还包括len,capacity,name等。从这点上看，所有java对象变量都是引用，而rust需要通过&操作符获取引用。

引用不具有原变量的所有权，因此不需要返回引用来返回所有权。通过引用传参被称为borrowing。

## 可修改引用

首先引用指向的变量需要是可修改的。

其次，为在编译器组织数据竞争，在*同一作用域*内，

1. **可修改引用**至多只能有一个
2. **不可修改引用**可以有多个
3. 不可同时存在**可修改引用* 和 **不可修改引用**

> 数据竞争：
> 两个或多个同时访问同一个数据
> 至少一次访问是写
> 访问之间没用同步机制

> *引用的作用域：*
> a reference's scopte starts from where it is introduced and continues through the last time that reference is used.

*使用引用的规则:*

+ 任意时刻，存在任意多个不可修改引用或一个可修改引用
+ 引用必须有效(即不可 dangling references)

## slice

语法：&s[starting_index..ending_index]
类似于View视图的概念，string slice的类型是&str，是不可修改引用。
对于i32类型数组的slice，类型是&[i32]。
