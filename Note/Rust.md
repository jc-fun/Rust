# Rust

## 1、入门指南

```rust
println!("hello world") // 打印文本
```

+ rust的缩进是4个空格，而不是tab
+ println!是一个rust macro，如果是函数的话，就没有!
+ rust文件命名规则为下划线命名法
+ cargo build [--release]
  + 编译rust源代码并生成可执行文件，--release会在编译时对代码进行优化
+ cargo check
  + 检查代码，确保代码可以通过编译，但是不生成可执行文件
+ cargo run
  + 编译rust源代码并执行可执行文件，如果已经对源代码进行过编译且没有对源代码进行修改，这会直接跳过编译，执行可执行文件

## 2、猜数游戏

```rust
use rand::Rng;
use std::io; // trait
             // use rand::thread_rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数！");
    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64 u64
    println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // arm
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

```



+ 一般情况下，rust会将prelude模块导入到每一个程序的作用域中
+ 使用use关键字导入需要的库，例如：`use std::io;`
+ let关键字用来声明一个变量，但是这个变量默认是不可变的，如果想要修改这个变量，需要在let后面加上`mut`（mutable）关键字，例如：`let mut a = 1;`
+ String::new()将返回一个空白的字符串，new()是String类型的关联函数，类似于Java中的静态方法
+ io::stdin()将返回一个Stdin类型的实例，该实例会被用作句柄（handle）用来处理终端中的输入
+ `&mut`表示获取guess的引用，rust中引用默认同样是不可变得，因此同样需要加上mut
+ read_line方法会返回io::Result\<usize\>类型，这个类型是一个enums类型，其共有两个值：Ok、Err，当程序发生异常时，会调用expect方法，并将其中的字符串显示给用户
+ rand::Rng是一个`trait`，类似于Java中的接口，通过rand::thread_rng()返回Rng的实现ThreadRng类型
+ match表达式可以通过guess.cmp()返回的Ordering类型（enums类型）的值来决定下一步执行的分支（arm），因此也常被用作处理可能会发生的异常的手段
+ loop表达式表示无限循环，不要使用while true来实现无限循环，while循环侧重于while condition，而不是直接的while true，这更符合while的语义
+ Err(\_)，\_为通配符，因为这里不需要错误信息，因此使用通配符表示

## 3、通用编程概念

### 3.1 变量与可变性

```rust
const MAX_POINTS: u32 = 100_000;
println!("The maximum number of {}", MAX_POINTS);

let mut x = 5;
println!("The value of x is {}", x);

x = 6;
println!("The value of x is {}", x);

let y = 5;
let y = y + 1;
let y = y * 2;
println!("The value of y is {}", y);

let spaces = "        ";
let spaces = spaces.len();
println!("{}", spaces);
```



#### 3.1.1 变量

+ 声明变量使用`let`关键字
+ 默认情况下，变量是不可变的（Immutable）
+ 声明变量时，在变量前加上`mut`（mutable）关键字，就可以使变量可变

#### 3.1.2 常量（constant）

+ 常量在绑定值之后也是不可变的，但是它与不可变的变量有很多区别：
  + 常量不可以使用`mut`关键字修饰，常量永远都是不可变的
  + 声明常量使用`const`关键字，其类型必须被标注
  + 常量可以在任何作用域内进行声明，包括全局作用域
  + 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
+ 在程序运行期间，常量在其声明的作用域内一直有效
+ Rust中常量使用全大写字母，每个单词之间用下划线分开
+ 例子：`const MAX_POINTS : u32 = 100_000;`

#### 3.1.3 隐藏（shadowing）

+ 可以使用相同的名字声明新的变量，新的变量就会shadow（隐藏）之前声明的同名变量
+ shadow和把变量标记为`mut`是不一样的：
  + 如果不使用`let `关键字，那么重新给非`mut`的变量赋值会导致编译时错误
  + 而使用`let`声明的同名新变量，也是不可变的
  + 使用`let`声明的同名新变量，其类型可以与之前的不同



### 3.2 数据类型

#### 3.2.1 标量类型

> 一个标量类型代表一个单个的值

> Rust有四个主要的标量类型：
>
> + 整数类型
> + 浮点类型
> + 布尔类型
> + 字符类型

+ **整数类型**

  + 整数类型没有小数部分

  + 例如u32就是一个无符号的整数类型，占据32位的空间

  + 无符号整数类型以u开头

  + 有符号整数类型以i开头

  + Rust的整数类型列表如下：

    | Length  | Signed  | Unsigned |
    | ------- | ------- | -------- |
    | 8-bit   | `i8`    | `u8`     |
    | 16-bit  | `i16`   | `u16`    |
    | 32-bit  | `i32`   | `u32`    |
    | 64-bit  | `i64`   | `u64`    |
    | 128-bit | `i128`  | `u128`   |
    | arch    | `isize` | `usize`  |

  + isize和usize类型

    + isize和usize类型的位数由程序运行的计算机的架构所决定：如果是64位的计算机，那就是64位的
    + 使用isize和usize的主要场景是对某种集合进行索引操作

  + 整数字面值

    | Number Literals               | Example       |
    | ----------------------------- | ------------- |
    | Decimal (十进制)              | `98_222`      |
    | Hex (十六进制)                | `0xff`        |
    | Octal (八进制)                | `0o77`        |
    | Binary (二进制)               | `0b1111_0000` |
    | Byte (单字节字符)(仅限于`u8`) | `b'A'`        |

    + 除了byte类型外，所有的数值字面值都允许使用类型后缀：例如 57u8
    + 如果你不太清楚应该使用哪种类型，可以使用Rust响应的默认类型
    + 整数的默认类型就是i32：总体上来说i32的速度很快，即使在64位系统中

  + 整数溢出

    + 例如：u8的范围是0-255，如果把一个u8变量的值设为256，那么：
      + 调试模式下编译：Rust会检查整数溢出，如果发生溢出，程序在运行时就会panic
      + 发布模式下（--release）编译：Rust不会检查可能导致panic的整数溢出
        + 如果发生溢出：Rust会执行“环绕”操作：256变成0，257变成1...

  ```rust
  let guess: u32 = "42".parse().expect("Not a number");
  println!("{}", guess);
  ```

+ **浮点类型**
  + Rust有两种基础的浮点类型，也就是含有小数部分的类型
    + f32，32位，单精度
    + f64，64位，双精度
    
  + Rust的浮点类型使用了IEEE-754标准来表述
  
  + f64是默认类型，因为在现代CPU上f64和f32的速度差不多，而且精度更高
  
    ```rust
    let x = 2.0;
    let y: f32 = 3.0;
    ```

+ 数值操作
  + 加减乘除余

+ **布尔类型**
  + Rust布尔类型也有两个值：true和false
  
  + 一个字节大小
  
  + 符号是`bool`
  
    ```rust
    let t = true;
    let f: bool = false;
    ```

+ **字符类型**
  + Rust中国char类型被用来描述语言中最基础的单个字符
  
  + 字符类型的字面值使用单引号
  
  + 占用4个字节大小
  
  + 是Unicode标量值，可以表示比ASCII多得多的字符内容：拼音、中日韩文、零长度空白字符、emoji表情等
    + U+0000到U+D7FF
    + U+E000到U+10FFFF
    
  + 但是Unicode中并没有“字符”的概念，所以直接上认识的字符也许与Rust中的概念并不相符
  
    ```rust
    let x = 'x';
    let y: char = '卍';
    let z = '😂';
    ```

#### 3.2.2 复合类型

+ 复合类型可以将多个值放在一个类型里

+ Rust提供了两种基础的复合类型：元组（Tuple）、数组

+ **Tuple**
  + Tuple可以将多个类型的多个值放在同一个类型里
  
  + Tuple的长度是固定的：一旦声明就无法改变
  
  + 创建Tuple
    + 在小括号里，将值用逗号分开
    + Tuple中的每个位置都对应一个类型，Tuple中各元素的类型不必相同
    
  + 获取Tuple的元素值
    + 可以使用模式匹配来解构（*destructuring*）一个Tuple来获取元素的值
    
  + 访问Tuple的元素
    + 在Tuple变量使用点标记法，后接元素的索引号
    
  + 不带任何值的元组有个特殊的名称，叫做 **单元（unit）** 元组。这种值以及对应的类型都写作 `()`，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值
  
  ```rust
  let tup: (i32, f64, u8) = (500, 6.3, 1);
  
  let (x, y, z) = tup;
  println!("{}, {}, {}", x, y, z);
  
  println!("{}, {}, {}", tup.0, tup.1, tup.2);
  ```
  
+ **数组**
  + 数组也可以将多个值放在一个类型里
  
  + 数组中的每个元素的类型必须相同
  
  + 数组的长度也是固定的
  
  + 声明一个数组
    + 在中括号里，各值用逗号分开
    
  + 数组的用处
    + 如果想让你的数据存放在stack（栈）上而不是heap（堆）上，或者想保证有固定数量的元素，这时使用数组更有好处
    
  + 数组没有Vector灵活
    + Vector和数组类似，它由标准库提供
    + Vector的长度可以改变
    + 如果你不确定应该用数组还是Vector，那么估计你应该用Vector
    
  + 数组的类型
    + 数组的类型以这种形式表示：[类型; 长度]，例如： `let a: [i32; 5] = [1, 2, 3, 4, 5];`
    
  + 另一种声明数组的方法
    + 如果数组的每个元素值都相同，那么可以在：
      + 在中括号里指定初始值
      + 然后是一个 “ ; ”
      + 最后是数组的长度
      + 例如：`let a = [3; 5];`它就相当于：`let a = [3, 3, 3, 3, 3]`
    
  + 访问数组的元素
    + 数组是在Stack上分配的单个块的内存
    + 可以使用索引来访问数组的元素
    + 如果访问的索引超出了数组索引的范围，那么：
      + cargo check会通过
      + 编译运行时会报错（index out of bounds），Rust不会允许其继续访问相应地址的内存
    
    ```rust
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    
    let first = a[0];
    let second = a[1];
    let third = a[27]; // index out of bounds
    ```
    
    

### 3.3 函数与注释

#### 3.3.1 函数

+ 声明函数使用`fn`关键字

+ 针对函数和变量名，Rust使用snake case命名规范：所有的字母都是小写的，单词之间使用下划线分开

+ **函数的参数**
  + parameters、arguments
  
  + 在函数签名里，必须声明每个参数的类型
  
    ```rust
    fn another_function(x: i32, y: i32) {
        println!("Another function!");
        println!("the value of x and y is: {}, {}", x, y);
    }
    ```

+ **函数体中的语句与表达式**
  + 函数体由一系列语句组成，可选的有一个表达式结束
  
  + Rust是一个基于表达式的语言
  
  + 语句是执行一些动作的指令
  
  + 表达式会计算产生一个值
  
  + 函数的定义也是语句
  
  + 语句不返回值，所以不可以使用`let`将语句赋值 给一个变量
  
    ```rust
    // let x = (let y = 6); // expected expression, found statement (`let`)
    
    let y = {
        let x = 1;
        x + 3
    };
    ```

+ 函数的返回值
  + 在 `->`符号后面声明函数返回值的类型，但是不可以为返回值命名
  
  + 在Rust中，返回值就是函数体里面最后一个表达式的值
  
  + 若想提前返回，需使用return关键字，并指定一个值
    + 大多数函数都是默认使用最后一个表达式作为返回值
    
      ```rust
      fn five(x: i32) -> i32 {
          5 + x
      }
      let x = five(6);
      println!("The value of x is {}", x);
      ```

#### 3.3.2 注释

> 与C和Java一样



### 3.4 流程控制

#### 3.4.1 if表达式

+ `if`表达式允许你根据条件来执行不同的代码分支
  + 这个条件必须是bool类型
  
+ `if`表达式中，与条件相关联的代码块就叫做分支（arm）

+ 可选的，可以在后面加上一个`else`表达式

  ```rust
  let number = 3;
  
  if number < 5 {
      println!("condition was true");
  } else {
      println!("condition was false");
  }
  ```

+ 使用`else if`处理多重条件
  + 如果使用了多于一个`else if`，那么最好使用`match`来重构代码
  
+ 在`let`语句中使用`if`
  + 因为`if`是一个表达式，所以可以将它放在`let`语句中等号的右边
  
    ```rust
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}", number);
    ```

#### 3.4.2 循环

+ Rust提供了3种循环：`loop`、`while`和`for`

+ **loop循环**
  + `loop`关键字告诉Rust反复执行一块代码，直到你喊停
  
  + 可以在`loop`循环中使用`break`关键字来告诉程序何时停止循环
  
    ```rust
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    
    println!("The result is: {}", result);
    ```

+ **while条件循环**
  + 另外的一种常见的循环模式是每次执行循环体之前都判断一次条件
  
    ```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
    
        number = number - 1;
    }
    
    println!("LIFTOFF!!!")
    ```

+ **使用for循环遍历集合**
  + 可以使用`while`或`loop`来遍历集合，但是易错且低效
  
  + 使用`for`循环更简洁紧凑，它可以针对集合中的每个元素来执行一些代码
  
  + 由于`for`循环的安全、简洁性，所以它在Rust里用的最多
  
    ```rust
    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;
    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    ```

+ **Range**
  + 由标准库提供
  
  + 指定一个开始数字和一个结束数字，`Range`可以生成它们之间的数字（不含结束）
  
  + `rev`方法可以反转`Range`
  
    ```rust
    for number in (1 .. 4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    ```
  
    

## 4、认识所有权

### 4.1 什么是所有权

> 所有权是Rust最独特的特性，它让Rust无需GC就可以保证内存安全

+ Rust的核心特性就是所有权
+ 所有的程序在运行时都必须管理它们使用计算机内存的方式
  + 有些语言有垃圾回收机制，在程序运行时，它们会不断地寻找不再使用的内存
  + 在其它语言中，程序员必须显式地分配和释放内存
+ Rust采用了第三种方式
  + 内存通过一个所有权系统来管理的，其中包含一组编译器在编译时检查的规则
  + 当内存运行时，所有权特性不会减慢程序的运行速度

#### 4.1.1 Stack vs Heap

+ 在像 Rust 这样的系统编程语言中，值是位于栈上还是堆上在更大程度上影响了语言的行为以及为何必须做出这样的抉择
+ 在你的代码运行的时候，栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同
+ 栈以放入值的顺序存储值并以相反顺序取出值。这也被称作 **后进先出**（*last in, first out*）
  + 增加数据叫做 **进栈**（*pushing onto the stack*），而移出数据叫做 **出栈**（*popping off the stack*）
  + 栈中的所有数据都必须占用已知且固定的大小。
  + 在编译时大小未知或大小可能变化的数据，要改为存储在堆上。 
+ 堆是缺乏组织的
  + 当向堆放入数据时，你要请求一定大小的空间。
  + 内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 **指针**（*pointer*）。这个过程称作 **在堆上分配内存**（*allocating on the heap*），有时简称为 “分配”（allocating）内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 **指针**（*pointer*）。这个过程称作 **在堆上分配内存**（*allocating on the heap*），有时简称为 “分配”（allocating）
+ 把值压到stack上不叫分配，因为指针的大小是已知固定的，可以把指针存放在stack上
  + 但如果像要访问实际数据，则必须使用指针来定位
+ 入栈比在堆上分配内存要快
  + 因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶
  + 相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。
+ 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问
  + 对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
+ 出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。
  + 在heap上分配大量的空间也是需要时间的
+ 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

#### 4.1.2 所有权存在的原因

+ 所有权解决的问题
  + 跟踪代码的哪些部分正在使用heap的哪些数据
  + 最小化heap上的重复数据量
  + 清理heap上未使用的数据以避免空间不足
+ 一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的主要目的就是为了管理堆数据，这也能够帮助解释为什么所有权要以这种方式工作。

#### 4.1.3 所有权规则

+ 每个值都有一个变量，这个变量是该值的所有者（owner）
+ 每个值同时只能有一个所有者
+ 当所有者离开作用域（scope）时，该值将被删除

#### 4.1.4 变量作用域

+ `Scope`就是程序中一个项（item）在程序中的有效范围

  ```rust
  fn main() {
      // s 不可用
      let s = "hello"; // s 可用
      // 可以对 s 进行相关的操作
  } // 作用域到此结束，s 不再可用
  ```

#### 4.1.5 String类型

+ `String`比基础标量数据类型更复杂

+ 字符串字面值：程序里手写的哪些字符串值。它们是不可变的

+ Rust还有第二种字符串类型：String

  + `String`类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本

+ 创建`String`类型的值

  + 可以使用 `from` 函数基于字符串字面值来创建 `String`

  + `let s = String::from("hello")`

    + `::` 是运算符，允许将特定的 `from` 函数置于 `String` 类型的命名空间（namespace）下，而不需要使用类似 `string_from` 这样的名字，也就是`from`是`String`类型下的函数

    + 这类字符串是可以被修改的

      ```rust
      fn main() {
          let mut s = String::from("hello");
      
          s.push_str(", world");
      
          println!("{}", s);
      }
      ```

    + 为什么`String`类型的值可以修改，而字符串字面值却不能修改？
      + 因为它们处理内存的方式不同

#### 4.1.6 内存与分配

+ 就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中

  + 这使得字符串字面值快速且高效，这些特性都只得益于字符串字面值的不可变性

+ 对于 `String` 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容

  + 必须在运行时向内存分配器（memory allocator）请求内存

    + ​	这步通过调用`String::from`来实现

  + 当我们处理完 `String` 时，需要使用某种方式将内存返回给分配器

    + 在有 **垃圾回收**（*garbage collector*，*GC*）的语言中， GC 记录并清除不再使用的内存
    + 在大部分没有 GC 的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了
      + 如果忘记回收了会浪费内存
      + 如果过早回收了，将会出现无效变量，变量就会非法
      + 如果重复回收，这也是个 bug。我们需要精确的为一个 `allocate` 配对一个 `free`

  + Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。

    ```rust
    fn main() {
        {
            let s = String::from("hello"); // 从此处起，s 是有效的
            // 使用 s
        }                                  // 此作用域已结束       
    }                                      // s 不再有效
    ```
    
  + 当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)，在这里 `String` 的作者可以放置释放内存的代码。Rust 在结尾的 `}` 处自动调用 `drop`

#### 4.1.7 变量与数据交互的方式

> 在Rust 中，多个变量可以采取不同的方式与同一数据进行交互

+ 移动

  ```rust
  fn main() {
      let x = 5;
      let y = x;
  }
  ```
  
  整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
  
  ```rust
  fn main() {
      let s1 = String::from("hello");
      let s2 = s1;
  }
  ```
  
  情况和前面的例子不同
  
  + `String` 由三部分组成，这一组数据存储在栈上
  
    + 一个指向存放字符串内容内存的指针
    + 一个长度
    + 一个容量
  
  + 存放字符串内容的部分在heap上
  
  + 长度len，就是存放字符串内容所需的字节数
  
  + 容量capacity是指`String`从内存分配器总共获得的内存的总字节数
  
  ![String in memory](assets/String_in_memory.svg)
  
  + 当我们将 `s1` 赋值给 `s2`，`String` 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。
  
    ![the same value](the_same_value.svg)
  
  + 当变量离开作用域后，Rust 自动调用 `drop` 函数并清理变量的堆内存
  
  + 这就有了一个问题：当 `s2` 和 `s1` 离开作用域，他们都会尝试释放相同的内存
  
    + 这是一个叫做 **二次释放**（*double free*）的错误，也是之前提到过的内存安全性 bug 之一
    + 两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞
  
  + 确保内存安全
  
    + Rust没有尝试复制被分配的内存
    + Rust 认为 `s1` 不再有效
      + Rust 不需要在 `s1` 离开作用域后清理任何东西
  
    ```rust
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1;
    
        println!("{}, world!", s1);
    }
    ```
  
  + 你会得到一个类似如下的错误，因为 Rust 禁止你使用无效的引用
  
    ```rust
    error[E0382]: borrow of moved value: `s1`
     --> src\main.rs:5:28
      |
    2 |     let s1 = String::from("hello");
      |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    3 |     let s2 = s1;
      |              -- value moved here
    4 |
    5 |     println!("{}, world!", s1);
      |                            ^^ value borrowed here after move
      |
      = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    
    For more information about this error, try `rustc --explain E0382`.
    warning: `owner` (bin "owner") generated 1 warning
    error: could not compile `owner` due to previous error; 1 warning emitted
    ```
  
  + 如果你在其他语言中听说过术语 **浅拷贝**（*shallow copy*）和 **深拷贝**（*deep copy*），那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。不过因为 Rust 同时使第一个变量无效了，这个操作被称为 **移动**（*move*），而不是浅拷贝。上面的例子可以解读为 `s1` 被 **移动** 到了 `s2` 中。
  
    ![s1 moved to s2](assets/s1_moved_to_s2.svg)
  
  + 这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 **自动** 的复制可以被认为对运行时性能影响较小
  
+ 克隆

  + 如果我们 **确实** 需要深度复制 `String` 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 `clone` 的通用函数

    ```rust
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    ```

  + 只在栈上的数据：拷贝
  
    + 这里还有一个没有提到的小窍门。这些代码使用了整型并且是有效的
  
      ```rust
      fn main() {
          let x = 5;
          let y = x;
      
          println!("x = {}, y = {}", x, y);
      }
      ```
  
    + 但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 `clone`，不过 `x` 依然有效且没有被移动到 `y` 中
    + 原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 `y` 后使 `x` 无效。换句话说，这里没有深浅拷贝的区别，所以这里调用 `clone` 并不会与通常的浅拷贝有什么不同，我们可以不用管它。
    + Rust 有一个叫做 `Copy` trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上，如果一个类型实现了 `Copy` trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
    + 如果一个类型或者该类型的一部分实现类 `Drop`trait，那么Rust不允许让它再去实现`Copy`trait
      + 任何一组简单标量值的组合都可以实现 `Copy`
      + 任何不需要分配内存或某种形式资源的类型都可以实现 `Copy`
      + 一些拥有`Copy`trait的类型
        + 所有整数类型，比如 `u32`。
        + 布尔类型，`bool`，它的值是 `true` 和 `false`。
        + 所有浮点数类型，比如 `f64`。
        + 字符类型，`char`。
        + 元组，当且仅当其包含的类型也都实现 `Copy` 的时候。比如，`(i32, i32)` 实现了 `Copy`，但 `(i32, String)` 就没有。
  

#### 4.1.8 所有权与函数

+ 将值传递给函数与给变量赋值的原理相似

  + 将值传递给函数将发生移动或复制

  ```rust
  fn main() {
      let s = String::from("hello");  // s 进入作用域
  
      takes_ownership(s);             // s 的值移动到函数里 ...
                                      // ... 所以到这里不再有效
  
      let x = 5;                      // x 进入作用域
  
      makes_copy(x);                  // x 应该移动函数里，
                                      // 但 i32 是 Copy 的，
                                      // 所以在后面可继续使用 x
  
  } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 没有特殊之处
  
  fn takes_ownership(some_string: String) { // some_string 进入作用域
      println!("{}", some_string);
  } // 这里，some_string 移出作用域并调用 `drop` 方法。
    // 占用的内存被释放
  
  fn makes_copy(some_integer: i32) { // some_integer 进入作用域
      println!("{}", some_integer);
  } // 这里，some_integer 移出作用域。没有特殊之处
  ```

#### 4.1.9 返回值与作用域

+ 函数在返回值的过程中同样也会发生所有权的转移

  ```rust
  fn main() {
      let s1 = gives_ownership(); // gives_ownership 将返回值
                                      // 转移给 s1
  
      let s2 = String::from("hello"); // s2 进入作用域
  
      let s3 = takes_and_gives_back(s2); // s2 被移动到
                                         // takes_and_gives_back 中,
                                         // 它也将返回值移给 s3
  } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    // 所以什么也不会发生。s1 离开作用域并被丢弃
  
  fn gives_ownership() -> String {
      // gives_ownership 会将
      // 返回值移动给
      // 调用它的函数
  
      let some_string = String::from("yours"); // some_string 进入作用域.
  
      some_string // 返回 some_string
                  // 并移出给调用的函数
                  //
  }
  
  // takes_and_gives_back 将传入字符串并返回该值
  fn takes_and_gives_back(a_string: String) -> String {
      // a_string 进入作用域
      //
  
      a_string // 返回 a_string 并移出给调用的函数
  }
  ```

+ 一个变量的所有权总是遵循同样的模式

  + 把一个值赋给其它变量时就会发生移动
  + 当一个包含heap数据的变量离开作用域时，它的值就会被`drop`函数清理，除非数据的所有权移动到另一个变量上了

+ 如果我们想要函数使用一个值但不获取所有权该怎么办呢？

  ```rust
  fn main() {
      let s1 = String::from("hello");
  
      let (s2, len) = calculate_length(s1);
  
      println!("The length of '{}' is {}.", s2, len);
  }
  
  fn calculate_length(s: String) -> (String, usize) {
      let length = s.len(); // len() 返回字符串的长度
  
      (s, length)
  }
  ```

  但是这未免有些形式主义，而且这种场景应该很常见

+ Rust 对此提供了一个不用获取所有权就可以使用值的功能，叫做 **引用**（*references*）



### 4.2 引用和借用

#### 4.2.1 引用（reference）

+ 我们可以提供一个 `String` 值的引用（reference）。**引用**（*reference*）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 与指针不同，引用确保指向某个特定类型的有效值。

  ```rust
  fn main() {
      let s1 = String::from("hello");
  
      let len = calculate_length(&s1);
  
      println!("The length of '{}' is {}.", s1, len);
  }
  
  fn calculate_length(s: &String) -> usize {
      s.len()
  }
  ```

+ 我们传递 `&s1` 给 `calculate_length`，同时在函数定义中，我们获取 `&String` 而不是 `String`。这些 & 符号就是 **引用**，它们允许你使用值但不获取其所有权

  ![&String s pointing at String s1](assets/&String_s_pointing_at_String_s1.svg)

#### 4.2.2 借用（borrowing）

+ 我们将创建一个引用的行为称为 **借用**（*borrowing*）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。我们并不拥有它

+ 如果我们尝试修改借用的变量呢？

  ```rust
  fn main() {
      let s = String::from("hello");
  
      change(&s);
  }
  
  fn change(some_string: &String) {
      some_string.push_str(", world");
  }
  ```

  ```rust
  $ cargo run
     Compiling ownership v0.1.0 (file:///projects/ownership)
  error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
   --> src/main.rs:8:5
    |
  7 | fn change(some_string: &String) {
    |                        ------- help: consider changing this to be a mutable reference: `&mut String`
  8 |     some_string.push_str(", world");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  
  For more information about this error, try `rustc --explain E0596`.
  error: could not compile `ownership` due to previous error
  ```

  正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值（即不能乱修改借来的东西）。

#### 4.2.3 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

我们通过一个小调整就能允许我们修改一个借用的值，这就是 **可变引用**

+ 首先，我们必须将 `s` 改为 `mut`。然后在调用 `change` 函数的地方创建一个可变引用 `&mut s`，并更新函数签名以接受一个可变引用 `some_string: &mut String`。这就非常清楚地表明，`change` 函数将改变它所借用的值。

+ 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。这些尝试创建两个 `s` 的可变引用的代码会失败

  ```rust
  fn main() {
      let mut s = String::from("hello");
  
      let r1 = &mut s;
      let r2 = &mut s;
  
      println!("{}, {}", r1, r2);
  }
  ```

  ```rust
  $ cargo run
     Compiling ownership v0.1.0 (file:///projects/ownership)
  error[E0499]: cannot borrow `s` as mutable more than once at a time
   --> src/main.rs:5:14
    |
  4 |     let r1 = &mut s;
    |              ------ first mutable borrow occurs here
  5 |     let r2 = &mut s;
    |              ^^^^^^ second mutable borrow occurs here
  6 | 
  7 |     println!("{}, {}", r1, r2);
    |                        -- first borrow later used here
  
  For more information about this error, try `rustc --explain E0499`.
  error: could not compile `ownership` due to previous error
  ```

  这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用。新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是 Rust 可以在编译时就避免数据竞争。**数据竞争**（*data race*）类似于竞态条件，它可由这三个行为造成：

  + 两个或更多指针同时访问同一数据。
  + 至少有一个指针被用来写入数据。
  + 没有同步数据访问的机制。

+ 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

+ 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 **同时** 拥有

  ```rust
  fn main() {
      let mut s = String::from("hello");
  
      {
          let r1 = &mut s;
      } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
  
      let r2 = &mut s;
  }
  ```

+ **不可以同时拥有**一个可变引用和一个不可变的引用

  ```rust
  fn main() {
      let mut s = String::from("hello");
  
      let r1 = &s; // 没问题
      let r2 = &s; // 没问题
      let r3 = &mut s; // 大问题
  
      println!("{}, {}, and {}", r1, r2, r3);
  }
  ```

  ```rust
  $ cargo run
     Compiling ownership v0.1.0 (file:///projects/ownership)
  error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src/main.rs:6:14
    |
  4 |     let r1 = &s; // no problem
    |              -- immutable borrow occurs here
  5 |     let r2 = &s; // no problem
  6 |     let r3 = &mut s; // BIG PROBLEM
    |              ^^^^^^ mutable borrow occurs here
  7 | 
  8 |     println!("{}, {}, and {}", r1, r2, r3);
    |                                -- immutable borrow later used here
  
  For more information about this error, try `rustc --explain E0502`.
  error: could not compile `ownership` due to previous error
  ```

#### 4.2.4 悬空引用（Dangling References）

+ 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其它持有者了

+ 在 Rust 中编译器确保引用永远也不会变成悬垂状态

  + 当你引用了某些数据，编译器确保数据不会在其引用之前离开作用域

    ```rust
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String {
        let s = String::from("hello");
    
        &s
    }
    ```

    ```rust
    $ cargo run
       Compiling ownership v0.1.0 (file:///projects/ownership)
    error[E0106]: missing lifetime specifier
     --> src/main.rs:5:16
      |
    5 | fn dangle() -> &String {
      |                ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    help: consider using the `'static` lifetime
      |
    5 | fn dangle() -> &'static String {
      |                ~~~~~~~~
    
    For more information about this error, try `rustc --explain E0106`.
    error: could not compile `ownership` due to previous error
    ```

    我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免

    ```rust
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String { // dangle 返回一个字符串的引用
    
        let s = String::from("hello"); // s 是一个新字符串
    
        &s // 返回字符串 s 的引用
    } // 这里 s 离开作用域并被丢弃。其内存被释放。
      // 危险！
    ```

#### 4.2.5 引用的规则

- 在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
- 引用必须总是有效的。



### 4.3 Slice类型（切片）

> *slice* 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它不持有所有权

+ 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串

+ 我们并没有一个真正获取 **部分** 字符串的办法。不过，我们可以返回单词结尾的索引，结尾由一个空格表示

  ```rust
  fn main() {
      let mut s = String::from("hello world");
      let word_index = first_world(&s);
      s.clear(); // 这清空了字符串，使其等于 ""
      // word_index 在此处的值仍然是 5，
      // 但是没有更多的字符串让我们可以有效地应用数值 5。word_index 的值现在完全没有意义
      println!("{}", word_index)
  }
  
  fn first_world(s: &String) -> usize {
      let bytes = s.as_bytes();
  
      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return i;
          }
      }
      s.len()
  }
  ```

   `iter` 方法返回集合中的每一个元素，而 `enumerate` 包装了 `iter` 的结果，将这些元素作为元组的一部分来返回。`enumerate` 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。

   `enumerate` 方法返回一个元组，我们可以使用模式来解构。在 `for` 循环中，我们指定了一个模式，其中元组中的 `i` 是索引而元组中的 `&item` 是单个字节。因为我们从 `.iter().enumerate()` 中获取了集合元素的引用，所以模式中使用了 `&`。

  在 `for` 循环中，我们通过字节的字面值语法来寻找代表空格的字节。如果找到了一个空格，返回它的位置。否则，使用 `s.len()` 返回字符串的长度。

  我们返回了一个独立的 `usize`，不过它只在 `&String` 的上下文中才是一个有意义的数字。换句话说，因为它是一个与 `String` 相分离的值，无法保证将来它仍然有效。

  这个程序编译时没有任何错误，而且在调用 `s.clear()` 之后使用 `word` 也不会出错。因为 `word` 与 `s` 状态完全没有联系，所以 `word `仍然包含值 `5`。可以尝试用值 `5` 来提取变量 `s` 的第一个单词，不过这是有 bug 的，因为在我们将 `5` 保存到 `word` 之后 `s` 的内容已经改变。我们不得不时刻担心 `word` 的索引与 `s` 中的数据不再同步，这很啰嗦且易出错。

#### 4.3.1 字符串切片（slice）

+ **字符串 slice**（*string slice*）是 `String` 中一部分值的引用，它看起来像这样

  ```rust
  fn main() {
      let s = String::from("hello world");
  
      let hello = &s[0..5];
      let world = &s[6..11];
  }
  ```

+ 形式：[starting_index..ending_index]

  + starting_index就是切片起始位置的索引值
  + ending_index是切片终止位置的下一个索引值

  ![slice](assets/slice.svg)

  + 字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内
  + 如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出

+ 使用字符串切片重写例子

  ```rust
  fn main() { 
  
      let mut s = String::from("hello world");
      let word_index = first_word(&s);
  
      s.clear(); // 错误
      println!("{}", word_index);
  }
  
  fn first_word(s: &String) -> &str {
  
      let bytes = s.as_bytes();
  
      for(i, &item) in bytes.iter().enumerate() {
          if item == b' '{
              return &s[..i];
          }
      }
      &s[..]
  }
  ```

  ```rust
  $ cargo run
     Compiling ownership v0.1.0 (file:///projects/ownership)
  error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    --> src/main.rs:18:5
     |
  16 |     let word = first_word(&s);
     |                           -- immutable borrow occurs here
  17 | 
  18 |     s.clear(); // error!
     |     ^^^^^^^^^ mutable borrow occurs here
  19 | 
  20 |     println!("the first word is: {}", word);
     |                                       ---- immutable borrow later used here
  
  For more information about this error, try `rustc --explain E0502`.
  error: could not compile `ownership` due to previous error
  ```

  回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 `clear` 需要清空 `String`，它尝试获取一个可变引用。在调用 `clear` 之后的 `println!` 使用了 `word` 中的引用，所以这个不可变的引用在此时必须仍然有效。Rust 不允许 `clear` 中的可变引用和 `word` 中的不可变引用同时存在，因此编译失败。Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！

#### 4.3.2 字符串字面值就是 slice

+ 字符串字面值被直接存储在二进制程序中
  + `let word = "hello world";`
    + 变量 s 的类型是 `&str`，它是一个指向二进制程序特定位置的切片
    + `&str`是不可变引用，所以字符串字面值也是不可变的



#### 4.3.3 将字符串 slice作为参数传递

+ `fn first_word(s: &String) -> &str {`

+ 而更有经验的 Rustacean 会采用`&str`作为参数类型，因为这样就可以同时接收`String`和`&str`类型的参数了

  + `fn first_word(s: &str) -> &str {`
    + 使用字符串切片，直接调用该函数
    + 使用`String`，可以创建一个完整的`String`切片来调用该函数

+ 定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能

  ```rust
  fn main() {
      let my_string = String::from("hello world");
  
      // `first_word` 适用于 `String`（的 slice），整体或全部
      let word = first_word(&my_string[0..6]);
      let word = first_word(&my_string[..]);
      // `first_word` 也适用于 `String` 的引用，
      // 这等价于整个 `String` 的 slice
      let word = first_word(&my_string);
  
      let my_string_literal = "hello world";
  
      // `first_word` 适用于字符串字面值，整体或全部
      let word = first_word(&my_string_literal[0..6]);
      let word = first_word(&my_string_literal[..]);
  
      // 因为字符串字面值已经 **是** 字符串 slice 了，
      // 这也是适用的，无需 slice 语法！
      let word = first_word(my_string_literal);
  }
  ```

  

#### 4.3.4 其它类型的 slice

+ 就跟我们想要获取字符串的一部分那样，我们也会想要引用数组的一部分

  ```rust
  #![allow(unused)]
  fn main() {
  let a = [1, 2, 3, 4, 5];
  
  let slice = &a[1..3];
  
  assert_eq!(slice, &[2, 3]);
  }
  ```

  这个 slice 的类型是 `&[i32]`。它跟字符串 slice 的工作方式一样，通过存储集合中第一个元素的引用和一个集合总长度。你可以对其他所有集合使用这类 slice



## 5、使用结构体组织相关联的数据

> *struct*，或者 *structure*，是一个自定义数据类型，允许你包装和命名多个相关的值，从而形成一个有意义的组合。如果你熟悉一门面向对象语言，*struct* 就像对象中的数据属性。

### 5.1 结构体的定义和实例化

#### 5.1.1 创建struct

+ 使用`struct`关键字，并为整个`struct`命名

+ 在花括号内，为所有字段（Field）定义名称和类型

  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
  ```

#### 5.1.2 实例化struct

+ 想要使用`struct`，需要创建`struct`的实例

  + 为每个字段指定具体值
  + 无需按声明的顺序进行指定

  ```rust
  fn main() {
      let user1 = User {
          email: String::from("someone@example.com"),
          username: String::from("someusername123"),
          active: true,
          sign_in_count: 1,
      };
  }
  ```

#### 5.1.3 取得struct里面的某个值

+ 使用点标记法

  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  fn main() {
      let mut user1 = User {
          email: String::from("someone@example.com"),
          username: String::from("someusername123"),
          active: true,
          sign_in_count: 1,
      };
  
      user1.email = String::from("anotheremail@example.com");
  }
  ```

+ 一旦`struct`的实例是可变的，那么实例中所有字段都是可变的，Rust 并不允许只将某个字段标记为可变

+ 另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例

  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  fn build_user(email: String, username: String) -> User {
      User {
          email: email,
          username: username,
          active: true,
          sign_in_count: 1,
      }
  }
  
  fn main() {
      let user1 = build_user(
          String::from("someone@example.com"),
          String::from("someusername123"),
      );
  }
  ```

+ 为函数参数起与结构体字段相同的名字是可以理解的，但是不得不重复 `email` 和 `username` 字段名称与变量有些啰嗦。如果结构体有更多字段，重复每个名称就更加烦人了。幸运的是，有一个方便的简写语法！

#### 5.1.4 字段初始化简写语法

+ 当参数名与字段名都完全相同，我们可以使用 **字段初始化简写语法**（*field init shorthand*）来重写

  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  fn build_user(email: String, username: String) -> User {
      User {
          email,
          username,
          active: true,
          sign_in_count: 1,
      }
  }
  
  fn main() {
      let user1 = build_user(
          String::from("someone@example.com"),
          String::from("someusername123"),
      );
  }
  ```

#### 5.1.5 struct更新语法

+ 当你想基于某个`struct`实例来创建一个新实例的时候，可以使用 **结构体更新语法**（*struct update syntax*）实现

  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  fn main() {
      // --snip--
  
      let user1 = User {
          email: String::from("someone@example.com"),
          username: String::from("someusername123"),
          active: true,
          sign_in_count: 1,
      };
  	
      // 不使用更新语法
      let user2 = User {
          active: user1.active,
          username: user1.username,
          email: String::from("another@example.com"),
          sign_in_count: user1.sign_in_count,
      };
      
      //使用更新语法
      let user2 = User {
          email: String::from("another@example.com"),
          ..user1
      };
  }
  ```

#### 5.1.6 Tuple structs

+ 可定义类似`tuple`的`struct`，叫做元组结构体（*tuple structs*）

  + `Tuple structs`有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型
  + 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了

+ 定义 `Tuple structs`：使用`struct`关键字，后边是名字，以及里面元素的类型

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  fn main() {
      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0);
  }
  ```

+ `black` 和 `origin` 值的类型不同，因为它们是不同的元组结构体的实例

+ 元组结构体实例类似于元组，你可以将它们解构为单独的部分，也可以使用 `.` 后跟索引来访问单独的值

#### 5.1.7 没有任何字段的类单元结构体（*unit-like structs*）

+ 定义一个没有任何字段的结构体！它们被称为 **类单元结构体**（*unit-like structs*）,因为它们类似于 `()`，即“元组类型”中提到的 unit 类型

+ 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用

  ```rust
  struct AlwaysEqual;
  
  fn main() {
      let subject = AlwaysEqual;
  }
  ```

#### 5.1.8 struct数据的所有权

```rust
struct User {
active: bool,
username: &str,
email: &str,
sign_in_count: u64,
}
```

+ 这里的字段使用了`String`而不是`&str`
  + 该`struct`实例拥有其所有的数据
  + 只要`struct`实例是有效的，那么里面的字段数据也是有效的
  
+ `struct`里也可以存放引用，但这需要使用声明周期（lifetimes）
  + 生命周期保证只要`struct`实例是有效的，那么里面的引用也是有效的
  + 如果`struct`里面存储引用，而不使用生命周期就会报错
  
  ```rust
  struct User {
      active: bool,
      username: &str,
      email: &str,
      sign_in_count: u64,
  }
  
  fn main() {
      let user1 = User {
          email: "someone@example.com",
          username: "someusername123",
          active: true,
          sign_in_count: 1,
      };
  }
  
  ```
  
  ```rust
  $ cargo run
     Compiling structs v0.1.0 (file:///projects/structs)
  error[E0106]: missing lifetime specifier
   --> src/main.rs:3:15
    |
  3 |     username: &str,
    |               ^ expected named lifetime parameter
    |
  help: consider introducing a named lifetime parameter
    |
  1 ~ struct User<'a> {
  2 |     active: bool,
  3 ~     username: &'a str,
    |
  
  error[E0106]: missing lifetime specifier
   --> src/main.rs:4:12
    |
  4 |     email: &str,
    |            ^ expected named lifetime parameter
    |
  help: consider introducing a named lifetime parameter
    |
  1 ~ struct User<'a> {
  2 |     active: bool,
  3 |     username: &str,
  4 ~     email: &'a str,
    |
  
  For more information about this error, try `rustc --explain E0106`.
  error: could not compile `structs` due to 2 previous errors
  ```



### 5.2 结构体示例程序

使用 Cargo 新建一个叫做 *rectangles* 的二进制程序，它获取以像素为单位的长方形的宽度和高度，并计算出长方形的面积

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

```

这个示例代码在调用 `area` 函数时传入每个维度，虽然可以正确计算出长方形的面积，但我们仍然可以修改这段代码来使它的意义更加明确，并且增加可读性

这些代码的问题突显在 `area` 的签名上：

```rust
fn area(width: u32, height: u32) -> u32 {
```

函数 `area` 本应该计算一个长方形的面积，不过函数却有两个参数。这两个参数是相关联的，不过程序本身却没有表现出这一点。将长度和宽度组合在一起将更易懂也更易处理。第三章的元组类型 部分已经讨论过了一种可行的方法：元组。

#### 5.2.1 使用元组重构

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

在某种程度上说，这个程序更好一点了。元组帮助我们增加了一些结构性，并且现在只需传一个参数。不过在另一方面，这个版本却有一点不明确了：元组并没有给出元素的名称，所以计算变得更费解了，因为不得不使用索引来获取元组的每一部分

在计算面积时将宽和高弄混倒无关紧要，不过当在屏幕上绘制长方形时就有问题了！我们必须牢记 `width` 的元组索引是 `0`，`height` 的元组索引是 `1`。如果其他人要使用这些代码，他们必须要搞清楚这一点，并也要牢记于心。很容易忘记或者混淆这些值而造成错误，因为我们没有在代码中传达数据的意图

#### 5.2.2 使用结构体重构：赋予更多意义

我们使用结构体为数据命名来为其赋予意义。我们可以将我们正在使用的元组转换成一个有整体名称而且每个部分也有对应名字的结构体

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

这里我们定义了一个结构体并称其为 `Rectangle`。在大括号中定义了字段 `width` 和 `height`，类型都是 `u32`。接着在 `main` 中，我们创建了一个具体的 `Rectangle` 实例，它的宽是 30，高是 50

函数 `area` 现在被定义为接收一个名叫 `rectangle` 的参数，其类型是一个结构体 `Rectangle` 实例的不可变借用。第四章讲到过，我们希望借用结构体而不是获取它的所有权，这样 `main` 函数就可以保持 `rect1` 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 `&`

`area` 函数访问 `Rectangle` 实例的 `width` 和 `height` 字段（注意，访问对结构体的引用的字段不会移动字段的所有权，这就是为什么你经常看到对结构体的引用）。`area` 的函数签名现在明确的阐述了我们的意图：使用 `Rectangle` 的 `width` 和 `height` 字段，计算 `Rectangle` 的面积。这表明宽高是相互联系的，并为这些值提供了描述性的名称而不是使用元组的索引值 `0` 和 `1` 。结构体胜在更清晰明了

#### 5.2.3 通过派生 trait 增加实用功能

在调试程序时打印出 `Rectangle` 实例来查看其所有字段的值非常有用。示例 5-11 像前面章节那样尝试使用 `println!` 宏。但这并不行

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

当我们运行这个代码时，会出现带有如下核心信息的错误

```rust
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` 宏能处理很多类型的格式，不过，`{}` 默认告诉 `println!` 使用被称为 `Display` 的格式：意在提供给直接终端用户查看的输出。目前为止见过的基本类型都默认实现了 `Display`，因为它就是向用户展示 `1` 或其他任何基本类型的唯一方式。不过对于结构体，`println!` 应该用来输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提供一个 `Display` 实现来使用 `println!` 与 `{}` 占位符

但是如果我们继续阅读错误，将会发现这个有帮助的信息：

```rust
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

让我们来试试！现在 `println!` 宏调用看起来像 `println!("rect1 is {:?}", rect1);` 这样。在 `{}` 中加入 `:?` 指示符告诉 `println!` 我们想要使用叫做 `Debug` 的输出格式。`Debug` 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值

这样调整后再次运行程序后，仍然能看到一个错误

```rust
error[E0277]: `Rectangle` doesn't implement `Debug`
```

不过编译器又一次给出了一个有帮助的信息：

```rust
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust **确实** 包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。为此，在结构体定义之前加上外部属性 `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

另一种使用 `Debug` 格式打印数值的方法是使用 `dbg!` 宏。`dbg!` 宏接收一个表达式的所有权（与 `println!` 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

我们可以把 `dbg!` 放在表达式 `30 * scale` 周围，因为 `dbg!` 返回表达式的值的所有权，所以 `width` 字段将获得相同的值，就像我们在那里没有 `dbg!` 调用一样。我们不希望 `dbg!` 拥有 `rect1` 的所有权，所以我们在下一次调用 `dbg!` 时传递一个引用。下面是这个例子的输出结果：

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

我们可以看到第一点输出来自 *src/main.rs* 第 10 行，我们正在调试表达式 `30 * scale`，其结果值是60（为整数实现的 `Debug` 格式化是只打印它们的值）。在 *src/main.rs* 第 14行 的 `dbg!` 调用输出 `&rect1` 的值，即 `Rectangle` 结构。这个输出使用了更为易读的 `Debug` 格式。当你试图弄清楚你的代码在做什么时，`dbg!` 宏可能真的很有帮助!

除了 `Debug` trait，Rust 还为我们提供了很多可以通过 `derive` 属性来使用的 trait，他们可以为我们的自定义类型增加实用的行为

#### 5.2.4 总结

+ `std::fmt::Display`
+ `std::fmt::Debug`
+ `#[derive(Debug)]`
+ `{:?}`
+ `{:#?}`



### 5.3 struct 方法

#### 5.3.1 方法的创建规则

+ **方法**（method）与函数类似：它们使用 `fn` 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。

+ 不过方法与函数是不同的：

  + 它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文）
  + 第一个参数总是 `self`，它代表调用该方法的结构体实例

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
  
      println!(
          "The area of the rectangle is {} square pixels.",
          rect1.area()
      );
  }
  ```

+ 在`impl`块里定义方法

+ 方法的第一个参数可以是`&self`，也可以获得其所有权或可变借用。和其它参数一样

+ 更良好的代码组织

#### 5.3.2 方法调用的运算符

+ C/C++：`object -> something()` 就和 `(*object).something()`一样，需要先解引用（dereference）
+ Rust没有 `->`运算符
+ Rust会自动引用或解引用
  + 在调用方法时就会发生这种行为
+ 在调用方法时，Rust根据情况自动添加`&`、`&mut`或`*`，以便 object 可以匹配方法的签名
+ 下面两行代码效果相同
  + `p1.distance(&p2);`
  + `(&p1).distance(&p2);`

#### 5.3.3 方法参数

方法可以有多个参数

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

#### 5.3.4 关联函数

+ 可以在`impl`块里定义不把`self`作为第一个参数的函数，它们叫做关联函数（不是方法）

  + 例如：`String::from()`

+ 关联函数通常用于构造器

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  impl Rectangle {
      fn square(size: u32) -> Self {
          Self {
              width: size,
              height: size,
          }
      }
  }
  
  fn main() {
      let sq = Rectangle::square(3);
  }
  ```

+ `::`符号

  + 关联函数
  + 模块创建的命名空间

#### 5.3.4 多个 impl 块

+ 每个`struct`允许拥有多个`impl`块

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  
  impl Rectangle {
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
  
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
      let rect2 = Rectangle {
          width: 10,
          height: 40,
      };
      let rect3 = Rectangle {
          width: 60,
          height: 45,
      };
  
      println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
      println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  }
  ```

> 结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 `impl` 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。

但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具



## 6、枚举与模式匹配

**枚举**（*enumerations*），也被称作 *enums*。枚举允许你通过列举可能的 **成员**（*variants*） 来定义一个类型。

### 6.1 枚举的定义

+ IP地址：IPv4、IPv6

  ```rust
  enum IpAddrKind {
      V4,
      V6,
  }
  ```

  `V4` 和 `V6`，被称为枚举的 **成员**（*variants*）

#### 6.1.1 枚举值

可以像这样创建 `IpAddrKind` 两个不同成员的实例：

```rust
let four = IpAddrKind::V4
let six = IpAddrKind::V6
```

==注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开==。这么设计的益处是现在 `IpAddrKind::V4` 和 `IpAddrKind::V6` 都是 `IpAddrKind` 类型的。例如，接着可以定义一个函数来获取任何 `IpAddrKind`：

```rust
fn route(ip_kind: IpAddrKind) {}
```

现在可以使用任一成员来调用这个函数：

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

#### 6.1.2 将数据附加到枚举的变体中

使用枚举甚至还有更多优势。进一步考虑一下我们的 IP 地址类型，目前没有一个存储实际 IP 地址 **数据** 的方法；只知道它是什么 **类型** 的。考虑到已经在第五章学习过结构体了，你可能会像这样处理问题：

```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。`IpAddr` 枚举的新定义表明了 `V4` 和 `V6` 成员都关联了 `String` 值：

```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

优点：

+ 不需要额外使用`struct`
+ 每个变体可以拥有不同的类型以及关联的数据量

用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。如果我们想要将 `V4` 地址存储为四个 `u8` 值而 `V6` 地址仍然表现为一个 `String`，这就不能使用结构体了。枚举则可以轻易的处理这个情况：

```rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### 6.1.3 标准库中的IpAddr

事实证明存储和编码 IP 地址实在是太常见了以至于标准库提供了一个开箱即用的定义！让我们看看标准库是如何定义 `IpAddr` 的：它正有着跟我们定义和使用的一样的枚举和成员，不过它将成员中的地址数据嵌入到了两个不同形式的结构体中，它们对不同的成员的定义是不同的：

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚至可以包含另一个枚举！另外，标准库中的类型通常并不比你设想出来的要复杂多少。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### 6.1.4 为枚举定义方法

结构体和枚举还有另一个相似点：就像可以使用 `impl` 来为结构体定义方法那样，也可以在枚举上定义方法。这是一个定义于我们 `Message` 枚举上的叫做 `call` 的方法：

```rust
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

#### 6.1.5 Option枚举

+ 定义于标准库中
+ 在`Prelude`（预导入模块）中
+ 描述了：某个值可能存在（某种类型）或不存在的情况

Rust 并没有很多其他语言中有的空值功能。**空值**（*Null* ）是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。

Null引用：The Billion Dollar Mistake

空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。

空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。

问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 `Option<T>`，它在标准库中定义。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Option枚举包含在`Prelude`（预导入模块）中，可以直接使用：

+ `Option<T>`
+ `Some(T)`
+ `None`

`Option<T>` 比 `Null` 好在哪？

+ `Option<T>` 和 `T` 是不同的类型，不可以把 `Option<T>`直接当成`T`

  ```rust
  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  
  let sum = x + y;
  ```

  ```rust
  $ cargo run
     Compiling enums v0.1.0 (file:///projects/enums)
  error[E0277]: cannot add `Option<i8>` to `i8`
   --> src/main.rs:5:17
    |
  5 |     let sum = x + y;
    |                 ^ no implementation for `i8 + Option<i8>`
    |
    = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  
  For more information about this error, try `rustc --explain E0277`.
  error: could not compile `enums` due to previous error
  ```

错误信息意味着 Rust 不知道该如何将 `Option<i8>` 与 `i8` 相加，因为它们的类型不同。当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值。我们可以自信使用而无需做空值检查。只有当使用 `Option<i8>`（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。

换句话说，在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`



### 6.2 match控制流结构

Rust 有一个叫做 `match` 的极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。模式可由字面值、变量、通配符和许多其他内容构成。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

#### 6.2.1 绑定值的模式

匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的

```rust
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let state = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", state)
}

```

#### 6.2.2 匹配 Option\<T>

我们在之前的部分中使用 `Option<T>` 时，是为了从 `Some` 中取出其内部的 `T` 值；我们还可以像处理 `Coin` 枚举那样使用 `match` 处理 `Option<T>`！只不过这回比较的不再是硬币，而是 `Option<T>` 的成员，但 `match` 表达式的工作方式保持不变。

比如我们想要编写一个函数，它获取一个 `Option<i32>` ，如果其中含有一个值，将其加一。如果其中没有值，函数应该返回 `None` 值，而不尝试执行任何操作。

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

#### 6.2.3 match匹配必须穷举所有的可能

`match` 还有另一方面需要讨论：这些分支必须覆盖了所有的可能性。考虑一下 `plus_one` 函数的这个版本，它有一个 bug 并不能编译：

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

```rust
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Rust 知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了！Rust 中的匹配是 **穷尽的**（*exhaustive*）：必须穷举到最后的可能性来使代码有效。特别的在这个 `Option<T>` 的例子中，Rust 防止我们忘记明确的处理 `None` 的情况，这让我们免于假设拥有一个实际上为空的值，从而使之前提到的价值亿万的错误不可能发生。

#### 6.2.4 通配模式和 _ 占位符

我们希望对一些特定的值采取特殊操作，而对其他的值采取默认操作。这时，可以使用通配模式other或 _ 占位符来替代其余没列出的值。

```rust
fn main() {
    let v = 1u8;

    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _other => println!("{}", _other),
    }

    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```

`other`和`_`的作用是类似的，只不过使用`other`可以获取到值。



### 6.3 if let 简洁控制流

`if let`处理只关心一种匹配而忽略其它匹配的情况

```rust
fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    }
}
```

`if let` 语法获取通过等号分隔的一个模式和一个表达式。它的工作方式与 `match` 相同，这里的表达式对应 `match` 而模式则对应第一个分支。

使用 `if let` 意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 `match` 强制要求的穷尽性检查。`match` 和 `if let` 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

可以认为 `if let` 是 `match` 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

可以在 `if let` 中包含一个 `else`。`else` 块中的代码与 `match` 表达式中的 `_` 分支块中的代码相同，这样的 `match` 表达式就等同于 `if let` 和 `else`。 

```rust
fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
```



## 7、Package、Crate、Module

Rust的代码组织

+ 代码组织主要包括：

  + 哪些细节可以暴露，哪些细节是私有的

  + 作用域内哪些名称是有效的

+ 模块系统：

  + Package（包）：Cargo的特性，让你构建、测试、共享crate
  + Crate（单元包）：一个模块树，它可以产生一个library或可执行文件
  + Module（模块）、use：让你控制代码的组织、作用域、私有路径
  + Path（路径）：为`struct`、`function`或`module`等项命名的方式

### 7.1 Package和Crate

crate 是 Rust 在编译时最小的代码单位。如果你用 `rustc` 而不是 `cargo` 来编译一个文件（第一章我们这么做过），编译器还是会将那个文件认作一个 crate。 crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译。

#### 7.1.1 Crate

+ Crate的类型

  + binary

  + library

+ Crate Root

  + 是源代码文件

  + Rust编译器从这里开始，组成你的Crate的根Module

#### 7.1.2 Package

+ 包含1个`Cargo.toml`，它描述了如何构建这些Crates
+ 只能包含 0 - 1个 library crate
+ 可以包含任意数量的 binary crate
+ 但必须至少包含一个 crate （library 或 binary）

#### 7.1.3 Cargo的惯例

+ src / main.rs
  + binary crate 的 crate root
  + crate 名与 package 名相同
+ src / lib.rs
  + package 包含一个 library crate
  + library crate 的 crate root
  + crate 名与 package 名相同
+ Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary
+ 一个 Package 可以同时包含 src / main.rs 和 src / lib.rs
  + 一个 binary crate，一个 library crate
  + 名称与 package 名相同
+ 一个 Package 可以有多个 binary crate
  + 文件放在 src / bin下
  + 每个文件是单独的 binary crate

#### 7.1.4 Crate的作用

+ 将相关功能组合到一个作用域内，便于在项目间进行共享
  + 防止命名冲突
+ 例如 `rand crate`，访问它的功能需要通过它的名字：`rand`



### 7.2 定义module来控制作用域和私有性

#### 7.2.1 Module

+ 在一个 crate 内，将代码进行分组
+ 增加可读性，易于复用
+ 控制条目（item）私有性。`public`、`private`

#### 7.2.2 建立Module

+ 使用 `mod`关键字
+ module可以嵌套
+ 可包含其它项（struct、enum、常量、trait、函数等）的定义

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

`src/main.rs` 和 `src/lib.rs` 叫做 crate root。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 `crate` 的模块，该结构被称为 *模块树*（*module tree*）



### 7.3 路径（Path）

为了在Rust的模块中找到某个条目，需要使用路径。路径有两种形式：

- **绝对路径**（*absolute path*）从 crate root 开始，以 crate 名或者字面值 `crate` 开头。
- **相对路径**（*relative path*）从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。

绝对路径和相对路径都后跟一个或多个由双冒号（`::`）分割的标识符。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    
}
```

```rust
cr   
   Compiling path_demo v0.1.0 (E:\Files\Learning\BackEnd\Rust\Code\path_demo)
error[E0603]: module `hosting` is private
 --> src\lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src\lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src\lib.rs:11:21
   |
11 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src\lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `path_demo` due to 2 previous errors
```

错误信息说 `hosting` 模块是私有的。换句话说，我们拥有 `hosting` 模块和 `add_to_waitlist` 函数的的正确路径，但是 Rust 不让我们使用，因为它不能访问私有片段。

#### 7.3.1 私有边界（privacy boundary）

+ 模块不仅可以用组织代码，还可以定义私有边界
+ 如果想把函数或`struct`等设为私有，可以将它放到某个模块中
+ Rust中所有的条目（函数、方法、struct、enum、模块、常量）默认是私有的
+ 父级模块无法访问子模块中的私有条目
+ 子模块里可以使用所有祖先模块中的条目
+ 可以调用同级模块中的私有条目

#### 7.3.2 pub关键字

+ 使用 `pub`关键字来将某些条目标记为公共的

  ```rust
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }
  
  pub fn eat_at_restaurant() {
  
      crate::front_of_house::hosting::add_to_waitlist();
  
      front_of_house::hosting::add_to_waitlist();
      
  }
  ```

#### 7.3.3 super关键字

+ super关键字用来访问父级模块路径中的内容，类似文件系统中的 `..`

  ```rust
  fn serve_order() {}
  
  mod back_of_house {
  
      fn fix_incorrect_order() {
          cook_order();
          super::serve_order();
          crate::serve_order();
      }
  
      fn cook_order() {}
  }
  ```

#### 7.3.4 pub struct

+ 将`pub`关键字放在`struct`前

  + `struct`为公共的
  + `struct`的字段默认是私有的

+ `struct`的字段需要单独设置`pub`来变成共有的

  ```rust
  mod back_of_house {
      pub struct Breakfast {
          pub toast: String,
          seasonal_fruit: String,
      }
  
      impl Breakfast {
          pub fn summer(toast: &str) -> Breakfast {
              Breakfast {
                  toast: String::from(toast),
                  seasonal_fruit: String::from("peaches"),
              }
          }
      }
  }
  
  pub fn eat_at_restaurant() {
      let mut meal = back_of_house::Breakfast::summer("Rye");
      meal.toast = String::from("Wheat");
      println!("I' d like {} toast please", meal.toast);
      meal.seasonal_fruit = String::from("blueberries"); // 无法访问私有字段
  }
  ```

#### 7.3.5 pub enum

+ 将`pub`关键字放在`enum`前

  + `enum`是公共的
  + `enum`的变体也都是公共的

  ```rust
  mod back_of_house {
      pub enum Appetizer {
          Soup,
          Salad,
      }
  }
  ```

  

### 7.4 use关键字

+ `use`关键字可以将路径导入到作用域内

  + 任遵守私有性规则

  ```rust
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
          fn some_function() {}
      }
  }
  use crate::front_of_house::hosting;
  
  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      // hosting::some_function();
  }
  ```

+ 使用`use`来指定相对路径

#### 7.4.1 use的习惯用法

+ 函数：将函数的父级模块引入作用域（指定到父级）

+ `struct`、`enum`、其它：指定完整路径（指定到本身）

+ 同名条目：指定到父级

+ 可以使用 `as`关键字提供新的名称

  ```rust
  use std::fmt;
  use std::io;
  
  fn f1() -> fmt::Result {}
  
  fn f2() -> io::Result {}
  
  
  
  use std::fmt::Result as fmtResult;
  use std::io::Result as ioResult;
  
  fn f1() -> fmtResult {}
  
  fn f2() -> ioResult {}
  ```

#### 7.4.2 使用 pub use 重新导出名称

+ 使用`use`将路径（名称）导入到作用域内后，该名称在此作用域内是私有的

+ `pub use`：重导出

  + 将该条目引入作用域
  + 该条目可以被外部代码引入到它们的作用域

  ```rust
  pub use crate::front_of_house::hosting;
  ```

#### 7.4.3 使用外部包（package）

+ 导入过程

  1. `Cargo.toml`添加依赖的包（package）
     + <a href = "https://crates.io/">https://crates.io/</a> 

  1. 使用`use`将特定条目引入作用域

     ```rust
     Cargo.toml :
     [dependencies]
     rand = "0.8.5"
     
     main.rs : 
     use rand::Rng;
     ```

+ 标准库（std）也被当做外部包

  + 不需要修改`Cargo.toml`来包含`std`

  + 需要使用`use`将`std`中的特定条目引入当前作用域

    ```rust
    use std::collections::HashMap;
    ```

#### 7.4.4 使用嵌套路径清理大量的use语句

+ 如果使用同一个包或模块下的多个条目

  ```rust
  use std::cmp::Ordering;
  use std::io;
  ```

+ 可使用嵌套路径在同一行内将上述条目进行引入

  + `路径相同的部分::{路径差异的部分}`

  ```rust
  use std::cmp::Ordering;
  use std::io;
  use std::{cmp::Ordering, io};
  ```

+ 如果两个`use`路径之一是另一个的子路径

  + 使用`self`

  ```rust
  use std::io;
  use std::io::Write;
  use std::io::{self, Write};
  ```

#### 7.4.5 通配符 *

+ 使用`*`可以把路径中所有的公共条目都引入到作用域

  ```rust
  use std::collections::*;
  ```

  

### 7.5 将模块拆分为不同文件

#### 7.5.1 将模块内容移动到其它文件

+ 模块定义时，如果模块名后边是`;`，而不是代码块
  + Rust会从与模块同名的文件中加载内容
  + 模块树的结构不会发生变化
+ 随着模块逐渐变大，该技术让你可以把模块的内容移动到其它文件中



## 8、常用的集合

Rust 标准库中包含一系列被称为 **集合**（*collections*）的非常有用的数据结构。大部分其他数据类型都代表一个特定的值，不过集合可以包含多个值。不同于内建的数组和元组类型，这些集合指向的数据是储存在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小。

- *vector* 允许我们一个挨着一个地储存一系列数量可变的值
- **字符串**（*string*）是字符的集合。我们之前见过 `String` 类型，不过在本章我们将深入了解。
- **哈希 map**（*hash map*）允许我们将值与一个特定的键（key）相关联。这是一个叫做 *map* 的更通用的数据结构的特定实现。

### 8.1 使用Vector存储多个值

+ `Vec<T>`，叫做`Vector`
  + 由标准库提供
  + 可存储多个值
  + 只能存储相同类型的数据
  + 值在内存中连续存放

#### 8.1.1 创建Vector

+ `Vec::new`函数

  ```rust
  let v: Vec<i32> = Vec::new();
  ```

+ 使用初始值创建`Vec<T>`，使用`vec!`宏

  ```rust
  let v = vec![1, 2, 3];
  ```

#### 8.1.2 更新Vector

+ 向`Vector`添加元素，使用`push`方法

  ```rust
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v.push(4);
  ```

#### 8.1.3 删除Vector

+ 与任何其它`struct`一样，当`Vector`离开作用域后

  + 它就被清理掉了
  + 它所有的元素也被清理掉了

  ```rust
  {
      let v = vec![1, 2, 3, 4];
  
      // 处理变量 v
  } // <- 这里 v 离开作用域并被丢弃
  ```

#### 8.1.4 读取Vector的元素

+ 两种方式可以引用`Vector`里的值

  + 索引
  + `get`方法

+ 索引 vs `get`处理访问越界

  + 索引：panic
  + `get`：返回None

  ```rust
  
  let v = vec![1, 2, 3, 4];
  let third: &i32 = &v[2];
  println!("The third element is: {}", third);
  
  match v.get(2) {
      Some(third) => println!("The third element is: {}", third),
      None => println!("There is no third element"),
  }
  ```

#### 8.1.5 所有权和借用规则

+ 不能在同一个作用域内同时拥有可变和不可变引用

  ```rust
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  v.push(6);
  println!("The first element is: {}", first);
  ```

  ```rust
  error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  ```

  为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

#### 8.1.6 遍历Vector中的值

+ for循环

  ```rust
  let v = vec![1, 2, 3, 4, 5];
  for i in &v {
      println!("{}", i);
  }
  
  let mut v = vec![1, 2, 3, 4, 5];
  for i in &mut v {
      *i += 50;
  }
  for i in &v {
      println!("{}", i);
  }
  ```

#### 8.1.7 使用枚举来存储多种数据类型

+ `Enum`的变体可以附加不同类型的数据

+ `Enum`的变体定义在同一个`enum`类型下

  ```rust
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }
  
  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(11.22),
      SpreadsheetCell::Text(String::from("bule")),
  ];
  ```
  
  Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。第二个好处是可以准确的知道这个 vector 中允许什么类型。如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
  
  

### 8.2 字符串

字符串是新晋 Rustacean 们通常会被困住的领域，这是由于三方面理由的结合：Rust 倾向于确保暴露出可能的错误，字符串是比很多程序员所想象的要更为复杂的数据结构，以及 UTF-8。所有这些要素结合起来对于来自其他语言背景的程序员就可能显得很困难了。

#### 8.2.1 什么是字符串

+ 特点

  + `Byte`的集合

  + 提供了一些方法
    + 可以将`byte`解析为文本

+ Rust的**核心语言层面**，只有一个字符串类型：字符串切片`str`（或`&str`）

+ 字符串切片：对存储在其它地方，UTF-8编码的字符串的引用

  + 字符串字面值：存储在二进制文件中，也是字符串切片

+ `String`类型

  + 来自**标准库**而不是核心语言
  + 可增长、可修改、可拥有
  + UTF-8编码

+ 通常说的字符串是指`String`和`&str`两种类型

  + 标准库里用的多
  + UTF-8编码

+ 其它类型的字符串：OsString、OsStr、CString、CStr

  + `String` vs `Str`后缀：拥有或借用的变体
  + 可存储不同编码的文本或在内存中以不同的形式展现

+ Library crate针对存储字符串可提供更多的选项

#### 8.2.2 创建一个新的字符串（String）

+ 很多`Vec<T>`的操作都可以用于`String`

+ `String::new()`函数

  ```rust
  let mut s = String::new();
  ```

+ 使用初始值来创建`String`

  + `to_string()`方法，可用于实现了`Display trait`的类型，包括字符串字面值

    ```rust
    let data = "initial contents";
    let s = data.to_string();
    
    let s1 = "initial contents".to_string();
    ```

  + `String::from()`函数，从字面值创建`String`

    ```rust
    let s2 = String::from("initial contents");
    ```

  + UTF-8

    ```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    ```

#### 8.2.3 更新String

+ `push_str()`方法：可以将一个字符串切片附加到`String`

  ```rust
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("{}", s);
  
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);
  ```

+ `push()`方法：把单个字符附加到`String`

  ```rust
  let mut s = String::from("foo");
  s.push_str("bar");
  s.push('!');
  println!("{}", s);
  ```

+ `+`：连接字符串

  + 使用了类似这个签名的方法`fn add(self, s: &str) -> String {...}`
    + 标准库中的`add`方法使用了泛型
    + 只能把`&str`添加到`String`
    + 解引用强制转换（deref coercion）

  ```rust
  let s1 = String::from("Hello, ");
  let s2 = String::from("world");
  let s3 = s1 + &s2;
  println!("{}", s3);
  println!("{}", s1); // 注意 s1 被移动了，不能继续使用
  println!("{}", s2);
  ```

+ `format!`：连接多个字符串

  + 和`println!()`类似，但返回字符串
  + 不会获得参数的所有权

  ```rust
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  
  // let s3 = s1 + "-" + &s2 + "-" + &s3;
  // println!("{}", s3);
  
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);
  ```

#### 8.2.4 对String按索引的形式进行访问

+ 按索引语法访问`String`的某部分，会报错

  ```rust
  let s1 = String::from("hello");
  let h = s1[0];
  ```

  ```rust
  error[E0277]: the type `String` cannot be indexed by `{integer}`
  ```

  Rust的字符串是不支持索引语法访问的

+ 内部表示

  + `String`是对`Vec<u8>`的包装

    + `len()`方法

    ```rust
    let len = String::from("Hola").len();
    println!("{}", len); // 4
    
    let len = String::from("Здравствуйте").len();
    println!("{}", len); // 24
    
    let hello = "Здравствуйте";
    let answer = &hello[0];
    // З: 208 151
    ```

    当使用 UTF-8 编码时，`З` 的第一个字节 `208`，第二个是 `151`，所以 `answer` 实际上应该是 `208`，不过 `208` 自身并不是一个有效的字母。返回 `208` 可不是一个请求字符串第一个字母的人所希望看到的，不过它是 Rust 在字节索引 0 位置所能提供的唯一数据。用户通常不会想要一个字节值被返回，即便这个字符串只有拉丁字母： 即便 `&"hello"[0]` 是返回字节值的有效代码，它也应当返回 `104` 而不是 `h`。

    为了避免返回意外的值并造成不能立刻发现的 bug，Rust 根本不会编译这些代码，并在开发过程中及早杜绝了误会的发生。

+ 字节、标量值和字形簇（Bytes、Scalar Values、Grapheme Clusters）

  + Rust有三种看待字符串的方式
    + 字节
    + 表量值
    + 字形簇（最接近“字母”的概念）

  ```rust
  let w = "नमस्ते"; // 梵文书写的印度语单词
  for b in w.bytes() { // 字节
      println!("{}", b);
  }
  
  for b in w.chars() { // 标量值
      println!("{}", b);
  }
  
  use unicode_segmentation::UnicodeSegmentation;
  for g in "नमस्ते".graphemes(true) { // 字形簇
      println!("{}", g);
  }
  ```

+ Rust不允许对`String`进行索引的最后一个原因

  + 索引操作应消耗一个常量时间 (O(1))
  + 而`String`无法保证：需要遍历所有内容，来确定有多少个合法的字符

#### 8.2.5 字符串slice

+ 可以使用`[]`和**一个范围**来创建字符串的切片

  + 必须谨慎使用
  + 如果切割时跨越了字符边界，程序就会`panic`

  ```rust
  let hello = "Здравствуйте";
  let s = &hello[0..4];
  println!("{}", s); // Зд
  
  let s1 = &hello[0..3];
  println!("{}", s1); // panic
  // byte index 3 is not a char boundary
  ```

#### 8.2.6 遍历String的方法

+ 对于标量值：`char()`方法
+ 对于字节：`byte()`方法
+ 对于字形簇：很复杂，标准库未提供

#### 8.2.7 String并不简单

+ Rust选择将正确处理`String`数据作为所有Rust程序的默认行为
  + 程序员必须在处理`UTF-8`数据之前投入更多的精力
+ 可以防止在开发后期处理涉及非`ASCII`字符的错误



### 8.3 HashMap<K, V>

`HashMap<K, V>` 类型储存了一个键类型 `K` 对应一个值类型 `V` 的映射。它通过一个 **哈希函数**（*hashing function*）来实现映射，决定如何将键和值放入内存中。

+ 键值对的形式存储数据，一个键（Key）对应一个值（Value）
+ `Hash`函数：决定如何在内存中存放`K`和`V`

#### 8.3.1 创建HashMap

+ 创建空`HashMap`：`new()`函数

+ 添加数据：`insert()`方法

  ```rust
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  
  ```

+ `HashMap`使用的比较少，因此不在`Prelude`中

+ 标准库对其支持比较少，没有内置的宏来创建`HashMap`

+ 数据存储在`heap`上

+ 同构的。一个`HashMap`中：

  + 所有的`K`必须是同一种类型
  + 所有的`V`必须是同一种类型

#### 8.3.2 另一种创建HashMap的方式：collect方法

+ 在元素类型为`Tuple`的`Vector`上使用`collect`方法，可以组建一个`HashMap`

  + 要求`Tuple`有两个值：一个作为`K`，一个作为`V`
  + `collect`方法可以把数据整合成很多种集合类型，包括`HashMap`
    + 返回值需要显式指明类型

  ```rust
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let intial_scores = vec![10, 50];
  
  let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
  
  println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
  ```

  `zip()`接收一个参数，将调用者中的元素与参数中的元素一一对应组成`Tuple`，若数量不匹配，多的元素会被丢弃。`collect()`方法形成了一个`HashMap`，元素顺序并不固定，每次运行可能都不一样。当然`key -> value`的顺序是由`zip`一一对应的，不是由`collect`决定的

#### 8.3.3 HashMap和所有权

+ 对于实现了`Copy trait`的类型（例如 i32），值会被复制到`HashMap`中

+ 对于拥有所有权的值（例如 String），值会被移动，所有权会被转移给`HashMap`

  ```rust
  let field_name = String::from("Favorite color");
  let field_valule = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(field_name, field_valule);
  
  println!("{}: {}", field_name, field_valule); // borrow of moved value
  ```

+ 如果将值的引用插入到`HashMap`，值本身不会移动

  + 在`HashMap`有效的期间，被引用的值必须保证有效

  ```rust
  let field_name = String::from("Favorite color");
  let field_valule = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(&field_name, &field_valule);
  
  println!("{}: {}", field_name, field_valule); // correct
  ```

#### 8.3.4 访问HashMap中的值

+ `get()`方法

  + 参数：`K`
  + 返回：`Option<&V>`

  ```rust
  let mut scores = HashMap::new();
  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  
  let team_name = String::from("blue");
  let score = scores.get(&team_name);
  
  match score {
      Some(s) => println!("{}", s),
      None => println!("team not exist"),
  }
  ```

#### 8.3.5 遍历HashMap

+ for循环

  ```rust
  let mut scores = HashMap::new();
  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  
  for (k, v) in &scores {
      println!("{}: {}", k, v);
  }
  ```

#### 8.3.6 更新HashMap<K, V>

+ `HashMap`大小可变

+ 每个`K`同时只能对应一个`V`

+ 更新`HashMap`中的数据

  + `K`已经存在，对应一个`V`
    + 替换现有的`V`
    + 保留现有的`V`，忽略新的`V`
    + 合并现有的`V`和新的`V`
  + `K`不存在
    + 添加一对新的`K, V`

+ 替换现有的`V`

  + 如果向`HashMap`插入一对`K、V`，然后在插入相同的`K`，但是`V`不同，那么原来的`V`会被替换掉

    ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 20);
    
    println!("{:?}", scores); // {"blue": 20}
    ```

+ 只有`K`不对应任何值的情况下，才插入`V`

  + `entry()`方法：检查指定的`K`是否对应一个`V`
    + 参数为`K`
    + 返回`enum Entry`：代表值是否存在
  + `Entry`的`or_insert()`方法：
    + 如果`K`存在，返回到对应的`V`的一个可变引用
    + 如果`K`不存在，将方法参数作为`K`的新值插进去，返回到这个值的可变引用

  ```rust
  let mut scores = HashMap::new();
  scores.insert(String::from("blue"), 10);
  // scores.entry(String::from("yellow")).or_insert(50);
  let e = scores.entry(String::from("yellow"));
  println!("{:?}", e);
  e.or_insert(50);
  
  scores.entry(String::from("blue")).or_insert(50);
  println!("{:?}", scores);
  ```

+ 基于现有的`V`来更新`V`

  ```rust
  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }
  println!("{:#?}", map); 
  // {
  //     "hello": 1,
  //     "world": 2,
  //     "wonderful": 1,
  // }
  ```

#### 8.3.7 Hash函数

+ 默认情况下，`HashMap`使用加密功能强大的`Hash`函数，可以抵抗拒绝服务（Dos）攻击
  + 不是可用的最快的`Hash`算法
  + 但具有更好的安全性
+ 可以指定不同的`hasher`来切换到另一个函数
  + `hasher`是实现了`BuildHasher trait`的类型



## 9、错误处理

错误是软件中不可否认的事实，所以 Rust 有一些处理出错情况的特性。在许多情况下，Rust 要求你承认错误的可能性，并在你的代码编译前采取一些行动。这一要求使你的程序更加健壮，因为它可以确保你在将代码部署到生产环境之前就能发现错误并进行适当的处理。

Rust 将错误分为两大类：**可恢复的**（*recoverable*）和 **不可恢复的**（*unrecoverable*）错误。对于一个可恢复的错误，比如文件未找到的错误，我们很可能只想向用户报告问题并重试操作。不可恢复的错误总是 bug 出现的征兆，比如试图访问一个超过数组末端的位置，因此我们要立即停止程序。

大多数语言并不区分这两种错误，并采用类似异常这样方式统一处理他们。Rust 没有异常。相反，它有 `Result<T, E>` 类型，用于处理可恢复的错误，还有 `panic!` 宏，在程序遇到不可恢复的错误时停止执行。

+ Rust的可靠性：错误处理
  + 大部分情况下：在编译时提示错误并处理
+ 错误的分类：
  + 可恢复
    + 例如文件未找到，可再次尝试
  + 不可恢复
    + bug，例如访问的索引超出范围
+ Rust没有类似异常的机制
  + 可恢复错误：`Result<T,E>`
  + 不可恢复：`panic!`宏

### 9.1 用 panic! 处理不可恢复的错误

+ 当`panic!`宏执行：
  + 你的程序会打印一个错误信息
  + 展开（unwind）、清理调用栈（Stack）
  + 退出程序

> ### 对应 panic 时的栈展开或终止
>
> 当出现 panic 时，程序默认会开始 **展开**（*unwinding*），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接 **终止**（*abort*），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。如果你需要项目的最终二进制文件越小越好，panic 时通过在 *Cargo.toml* 的 `[profile]` 部分增加 `panic = 'abort'`，可以由展开切换为终止。例如，如果你想要在release模式中 panic 时直接终止：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

+ 默认情况下，当`panic`发生：

  + 程序展开调用栈（工作量大）
    + Rust沿着调用栈往回走
    + 清理每个遇到的函数中的数据
  + 立即终止调用栈
    + 不进行清理，直接停止程序
    + 内存需要OS进行清理

+ 想让二进制文件更小，把设置从“展开”改为“终止”

  + 在`Cargo.toml`中适当的`profile`部分设置
    + `panic=abort`

  ```rust
  fn main() {
      // panic!("crash and burn");
  
      let v = vec![1, 2, 3];
      v[99];
  }
  ```

+ `panic!`可能出现在：

  + 我们写的代码中
  + 我们所依赖的代码中

+ 可以通过调用`panic!`的函数的回溯信息来定位引起问题的代码

  + 通过设置环境变量`RUST_BACKTRACE`可得到回溯信息

  ```rust
  export RUST_BACKTRACE=1 && cr                 [11:11:32]
      Finished dev [unoptimized + debuginfo] target(s) in 0.00s
       Running `target/debug/panic_demo`
  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:5
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
     1: core::panicking::panic_fmt
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
     2: core::panicking::panic_bounds_check
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:84:5
     3: <usize as core::slice::index::SliceIndex<[T]>>::index
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/slice/index.rs:250:10
     4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/slice/index.rs:18:9
     5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/vec/mod.rs:2628:9
     6: panic_demo::main
               at ./src/main.rs:5:5
     7: core::ops::function::FnOnce::call_once
               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  ```

  + 为了获取带有调试信息的回溯，必须启用调试符号（不带 `--release`）

### 9.2 用Result处理可恢复的错误

> 大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。

#### 9.2.1 Result枚举

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

+ `T`：操作成功情况下，`Ok`变体里返回的数据的类型

+ `E`：操作失败情况下，`Err`变体里返回的数据的类型

  ```rust
  let f = File::open("hello.txt");
  ```

#### 9.2.2 处理Result的一种方式：match表达式

+ 和`Option`枚举一样，`Result`及其变体也是由`prelude`带入作用域

  ```rust
  let f = File::open("hello.txt");
  
  let f = match f {
      Ok(file) => file,
      Err(error) => {
          panic!("error opening file: {:?}", error)
      }
  };
  ```

+ 匹配不同的错误

  ```rust
  let f = File::open("hello.txt");
  
  let f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
              Ok(fc) => fc,
              Err(e) => panic!("Error creating file: {:?}", e),
          },
          other_error => panic!("Error opening the file: {:?}", other_error),
      },
  };
  ```

  上例中使用了很多的`match!`，`match` 确实很强大，不过也非常的基础。第十三章我们会介绍闭包（closure），这可以用于很多 `Result<T, E>` 上定义的方法。在处理代码中的 `Result<T, E>` 值时这些方法可能会更加简洁。

  ```rust
  let f = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
          File::create("hello.txt").unwrap_or_else(|error| {
              panic!("Error creating file: {:?}", error);
          })
      } else {
          panic!("Error opening file: {:?}", error);
      }
  });
  ```

#### 9.2.3 unwrap

`match` 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。`Result<T, E>` 类型定义了很多辅助方法来处理各种情况。其中之一叫做 `unwrap`，它的实现就类似于 `match` 语句。如果 `Result` 值是成员 `Ok`，`unwrap` 会返回 `Ok` 中的值。如果 `Result` 是成员 `Err`，`unwrap` 会为我们调用 `panic!`。

+ `unwrap`相当于`match`表达式的一个快捷方法
  + 如果`Result`的结果是`Ok`，则返回`Ok`里面的值
  + 如果`Result`的结果是`Err`，则调用`panic!`宏

  ```rust
    let f = File::open("hello.txt").unwrap();
  ```

#### 9.2.4 expect

+ `expect`和`unwrap`类似，但是可以指定错误信息

    ```rust
    let f = File::open("hello.txt").expect("无法打开文件 hello.txt");
    ```

  

