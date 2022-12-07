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
    + 可以使用模式匹配来解构（destructure）一个Tuple来获取元素的值
    
  + 访问Tuple的元素
    + 在Tuple变量使用点标记法，后接元素的索引号
    
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

+ 移动

  + 在Rust 中，多个变量可以采取不同的方式与同一数据进行交互

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
  
  ![String in memory](assets\String in memory.svg)
  
  + 当我们将 `s1` 赋值给 `s2`，`String` 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。
  
    ![the same value](E:\Files\Learning\BackEnd\Rust\Note\assets\the same value.svg)
  
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
  
    ![s1 moved to s2](assets/s1 moved to s2.svg)
  
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
  
    
  
    
