# This Week in Rust #415：Rust与Linux信号

作者： 王江桐

> 本篇将会简要介绍什么是《This Week in Rust》，[第415篇推文](https://this-week-in-rust.org/blog/2021/11/03/this-week-in-rust-415/)中有关rustc安全漏洞，以及Rust与信号的内容。



## rustc安全漏洞

在11.1日，Rust公开了已修复的rustc安全漏洞CVE-2021-42574，详细介绍可参阅"[Security advisory for rustc (CVE-2021-42574)](https://blog.rust-lang.org/2021/11/01/cve-2021-42574.html)"或张汉东老师的博文《[特洛伊之源｜ 在 Rust 代码中隐藏无形的漏洞](https://zhuanlan.zhihu.com/p/428305373)》。 简要概括一下，整体情况大概为：

- Unicode编码同时支持从右往左以及从左往右的语言语序。为了支持在某个语序中书写相反语序的句子，Unicode额外支持名叫双向覆写的不可见代码点。

- 这些不可见代码点会在某些编译器和代码查看器中影响可见文本的输出，导致编译器编译的代码和用户看到的代码可能完全不同，从而导致实际执行的代码和用户看到代码并不一样。理论而言，这个漏洞实际上算是Unicode的漏洞而非rustc的漏洞，此类攻击方法对于C、C++、Java、Python等语言都有效。
- 为了解决这个安全问题，Rust1.56.1引入了两个新的lint来检测此类问题，并且拒绝潜藏问题的代码。如果代码中必须使用此类代码点，Rust编译器将提议使用对应的转义序列。同时，Rust项目组筛查了crate.io上所有的源码，保证这些代码不存在此类攻击。
- 但是这两个lint不能防止同形字符攻击，Rust编译器会认为这些方法是同名的并发出告警，需要用户自行注意。
- 此外，Rust 1.0.0以及1.56.0之间的版本则没有这个特性，如果不进行此项检查，代码可能遭受安全攻击。





## Linux信息

### 信号

信号（Signal）常用于Unix、类Unix、以及其他兼容便携式操作系统接口（POSIX）的操作系统，是进程间通信的一种方式，也是一种软中断，采用异步通信方式。信号由操作系统生成，发送给某个进程或同一进程内的某个线程，中断进程正常的控制流程，来触发某个事件。任何非原子操作在这个过程中都将被中断。此时，如果进程注册了该信号的处理函数，那么处理函数将被执行；没有处理函数时，程序可以忽略这个信号，或者执行默认的处理函数。不过，`SIGKILL`和`SIGSTOP`无法被截获并处理，这两个信号向内核和超级用户提供了进程终止和停止的可靠方法，如果忽略了，那么这个进程就变成了没人能管理的的进程，显然是内核设计者不希望看到的场景。

由于信号是异步的，在处理某一个信号的过程中，进程可能收到另一个信号，低级别信号会被高级别的信号中断，从而中断前一个信号的处理。信号可以造成进程中系统调用的中断，不过在信号处理完后会重新开始未完成的系统调用。因此，信号处理函数应该没有任何不想要的副作用，比如，`errno`的改变、信号掩码的改变、信号处理方法的改变，以及其他全局进程性质的改变。在信号处理函数内使用不可重入函数，如`malloc`和`printf`，也是不安全的。

信号可分为不可靠信号和可靠信号。不可靠信号意为信号可能会丢失，一旦信号丢失了，进程并不能知道信号丢失；相反，可靠信号，也称为阻塞信号，则不会丢失。当发送了一个阻塞信号，并且该信号的动作时系统默认动作或捕捉该信号，如果信号从发出以后会一直保持未决的状态，直到该进程对此信号解除了阻塞，或将对此信号的动作更改为忽略。

信号的阻塞和未决是通过信号的状态字来管理的，该状态字是按位来管理信号的状态。如果该信号被设置为阻塞的状态，也就是阻塞状态字对应位为1，那么信号未决状态字（pending）相应位会被内核设置为1；如果该信号阻塞解除了，也就是阻塞状态字设置为了0，那么信号未决状态字（pending）相应位会被内核设置为0，表示信号此时可以抵达了，也就是可以接收该信号了。阻塞状态字用户可以读写，未决状态字用户只能读，是由内核来设置表示信号递达状态的。每个信号都有独立的阻塞字，规定了当前要阻塞地达到该进程的信号集。

`sigprocmask()`系统调用可以阻塞和恢复信号的传递。在C中，`signal.h`头文件中包含了信号对应的正整数以及信号处理函数。

信号的计算和内存占用很小。信号类似于中断（Interrupt），不同之处在于中断由处理器调解并由内核处理，而信号由内核调解并由个体进程处理，内核调解可能通过系统调用。内核可以将中断作为信号传递给导致中断的进程，例如`SIGSEGV`、`SIGBUS`、`SIGILL`和`SIGFPE`。而对于异常，当其产生时，内核的异常处理程序有时不能解决问题，因此只能通过发送信号将异常交给进程自己处理。异常与信号的具体对应关系在不同的CPU架构上不同。



#### Linux常用不可靠信号

如下表格列出了Linux前31个信号，它们都是不可靠信号。信号的数值越小，则优先级越高。在Linux，可通过终端输入`kill -l`查看所有的signal信号。Linux信号的详解可见Linux manual page ["signal(7)"](https://man7.org/linux/man-pages/man7/signal.7.html)。

表中默认行为定义如下：

- 终止：进程异常终止。进程终止的结果和调用`_exit()`是一样的，除了终止可以向`wait()`和`waitpid()`返回导致进程终止的信号。

- 终止（核心转储）：进程异常终止。这种进程中止的过程根据实现有所不同，一般会创建一个核心文件。

- 忽略：进程忽略该信号。

- 暂停：进程被暂停，非终止。

- 继续：进程恢复执行。

  

|   信号    | 可移植代号 |    默认行为     |                             描述                             |
| :-------: | :--------: | :-------------: | :----------------------------------------------------------: |
|  SIGHUP   |     1      |      终止       |                             挂起                             |
|  SIGINT   |     2      |      终止       |                         终端中断信号                         |
|  SIGQUIT  |     3      | 终止 (核心转储) |                         终端退出信号                         |
|  SIGILL   |     4      | 终止 (核心转储) |                          非法的指令                          |
|  SIGTRAP  |     5      | 终止 (核心转储) |                        追踪/断点陷阱                         |
|  SIGABRT  |     6      | 终止 (核心转储) |                         进程终止信号                         |
|  SIGBUS   |     7      | 终止 (核心转储) |                    访问内存对象未定义区域                    |
|  SIGFPE   |     8      | 终止 (核心转储) |                        错误的算术运算                        |
|  SIGKILL  |     9      |      终止       |                杀死 (无法被捕获或忽略的信号)                 |
|  SIGUSR1  |     10     |      终止       |                       用户自定义信号1                        |
|  SIGSEGV  |     11     | 终止 (核心转储) |                        非法的内存引用                        |
|  SIGUSR2  |     12     |      终止       |                       用户自定义信号2                        |
|  SIGPIPE  |     13     |      终止       |                 写入一个没有连接另一端的管道                 |
|  SIGALRM  |     14     |      终止       |                          计时器告警                          |
|  SIGTERM  |     15     |      终止       |                           终止信号                           |
| SIGSTKFLT |     16     |      终止       | x86协处理器栈错误。但是因为协处理器不会出错，此信号不会隐式产生 |
|  SIGCHLD  |     17     |      忽略       |                    子进程终止、暂停、继续                    |
|  SIGCONT  |     18     |      继续       |                   如果被暂停，重新继续执行                   |
|  SIGSTOP  |     19     |      暂停       |              暂停执行（无法被捕获或忽略的信号）              |
|  SIGTSTP  |     20     |      暂停       |                         终端中止信号                         |
|  SIGTTIN  |     21     |      暂停       |                        后台进程尝试读                        |
|  SIGTTOU  |     22     |      暂停       |                        后台进程尝试写                        |
|  SIGURG   |     23     |      忽略       |            I/O有紧急数据（带外数据）到达当前进程             |
|  SIGXCPU  |     24     | 终止 (核心转储) |                       超出CPU时间限制                        |
|  SIGXFSZ  |     25     | 终止 (核心转储) |                       超出文件大小限制                       |
| SIGVTALRM |     26     |      终止       |                        虚拟定时器超时                        |
|  SIGPROF  |     27     |      终止       |                      性能调优定时器超时                      |
| SIGWINCH  |     28     |      忽略       |                       端窗口大小已变化                       |
|   SIGIO   |     29     |      终止       |        文件描述符准备就绪, 可以开始进行输入/输出操作         |
|  SIGPWR   |     30     |      终止       |                           电源错误                           |
|  SIGSYS   |     31     | 终止 (核心转储) |                        错误的系统调用                        |



#### Windows与信号

Windows系统实际上也有一套信号处理机制，或是某些机制处理产生信号的错误，但是并不完全与类Unix系统相同。例如，当用户按下`CTRL+C`时，在Linux系统下，`SIGINT`信号会被发送，但是在Win32系统环境下，操作系统会特殊生成一个线程来处理此信号。对于单线程应用来说，这会使得这个应用变成多线程，并且可能产生未知的错误。



### 中断

中断（Interrupt）是由硬件或软件所发送的一种称为IRQ（中断请求）的信号，所有的linux操作系统都是基于中断驱动的。中断事件由硬件或CPU产生，当发生时，会挂起正常的程序运行流程，并运行对应的中断处理器（Interrupt Handler）。当处理器运行完毕，之前的执行流程，也就是正常的程序运行流程，会被继续。

从来源上分类，中断可被分为两类：

- 软中断，由软件发送的中断请求信号，也被称为同步中断，一般由执行某个指令产生。软中断再分类，可分为两类：

  - 异常（Exception），通常是处理器在执行指令时，发现条件不符合要求，例如零除数或是某些系统调用。
  - 普通中断（Normal Interrupts）。

- 硬中断，由硬件发送的中断请求信号，也被称为异步中断，当I/O设备需要获取CPU资源或电源故障时产生。例如当包到达时，网卡会产生一个中断。从能否暂延或临时无效化中断上分类，硬中断可被分为另外两类：

  - 可屏蔽的（Maskable），大部分中断都可屏蔽，也就是说，我们可以延时运行中断处理器。

  - 不可屏蔽的（Non-Maskable）。

软中断和硬中断其他的一些差异可见下表：

| Hardware Interrupt                                | Software Interrupt                                           |
| :------------------------------------------------ | :----------------------------------------------------------- |
| 由外部设备或硬件产生，与外部硬件通信的一种方式    | 由计算机内部系统产生，与内核通信或产生系统调用的一种方式，尤其在处理错误或异常的时候 |
| 不增加程序计数器（Program Counter）               | 增加程序计数器                                               |
| 由外部设备产生，例如对开始I/O的请求，或是硬件失败 | 可由汇编指令产生                                             |
| 在所有中断中优先级最低                            | 在所有中断中优先级最高                                       |
| 异步事件                                          | 同步事件                                                     |
| 例：键盘输入或鼠标移动                            | 例：所有的系统调用                                           |



### 异常

异常（Exception）也被称为同步中断。异常的来源通常有两个：

- 执行指令时，处理器察觉到的程序条件异常，含三类：
  - 故障（Fault）
    - 故障通常在执行指令前被报告，并且通常可以被纠正，例如页错误（Page Fault）。在正常运行情况下，EIP寄存器里存储的是CPU下次要执行的指令的地址。在故障情况下，EIP中保存的是当前出错的指令地址，因此在纠正故障后，此条指令会被重新执行。
  - 陷阱（Trap）
    - 陷阱通常在执行指令后被报告，例如调试器陷阱。EIP寄存器中保存的是错误指令之后的下一个指令地址，而非当前地址。
  - 终止（Abort）
    - 终止通常意味着不可恢复的异常，会直接中断程序运行。
- 编程中出现的异常
  - 例如，`int n`





## Rust与信号

使用Rust生态表示信号以及对应处理机制是相对比较困难的。理由有三：

- 当收到信号时，当前运行线程被打断，转向运行信号处理函数。当信号处理函数运行结束，线程应当回到原来运行的位置继续运行。使用Rust的类型系统无法表示这种关系，因此无法安全地与主逻辑线程共享状态。
- 所有线程都应当共享信号处理函数，如果状态也是共享的，这会导致数据竞争。
- 在处理信号时，无法保证程序不会收到其他的信号。因此，信号处理函数中所有的指令都应当是可重入的，以便在之后恢复状态以及继续运行，或者是在接收到相同信号时，终止目前运行的实例，并且开始另一个新的同类实例的运行。然而，类似于`SIGSEGV`的信号可能会导致用户自定义的信号处理函数隐式调用其他不安全的函数。

Linux的[Man 7 signal-safety](https://man7.org/linux/man-pages/man7/signal-safety.7.html)给出了两种可能的解决方案：

- 保证信号处理函数只调用异步信号安全函数，并且对于主程序的全局变量而言，信号处理函数本身是可重入的。
- 信号处理函数或许可以获取某些全局数据。当使用不安全或处理这些全局数据的函数时，阻塞主程序获取信号。

第二个方案不适用于信号的真正处理。对于第一个方案，Rust确实会保证全局变量的修改是线程安全的，但是Rust并不保证函数是可重入的。通常情况下，建立与POSIX互斥锁之上的Rust互斥锁（Mutex）被用来保证全局变量修改的线程安全，然而对于同一个线程而言，如果试图对互斥锁再次进行上锁，那么就会陷入死锁的僵局。对于这点，更具体的讨论可见[Mutex.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/mutex.rs#L29)源码中注释。

因此，函数甚至是不可重入的。此外，对于第一点要求“异步信号安全”，锁并不在这个列表之中。锁、stdio、malloc、以及其他的一些函数都不是异步信号安全的。不过，使用Rust处理信号仍旧是可能的。"Working with signals in Rust"一文中除了分析以上内容，也给出了两个可能的实际解决方案：C FFI和Rust库，signal-hook。



### C FFI

在Rust中，可以使用C的函数来处理信号。例如，C中`signal`函数的接口如下：

```C
void (*signal(int sig, void (*func)(int)))(int);
```

Rust的libc对应接口如下：

```rust
pub unsafe extern "C" fn signal(
   signum: c_int, 
   handler: sighandler_t
) -> sighandler_t

type sighandler_t = size_t;
```

`extern`关键字表示函数来源于C。引号C（"C"）的部分定义了外部函数使用的应用二进制接口（Application Binary Interface, ABI），以及在汇编层面上如何调用该函数。其他的接口和Rust C FFI可见C的`signal.h`以及Rust [libc](https://crates.io/crates/libc)文档。

一个非常简单的例子是，如果想在用户按下`Ctrl + C`时告诉用户程序仍在执行，可以根据如上接口，注册如下信号处理器：

```rust
extern "C" fn handle_interrupt(sig: libc::c_int) { // 1
    println!("Sorry we didn't get the chance to finish");
}

fn main() {
    println!("Hello");

    // All libc functions are unsafe
    unsafe { 
        libc::signal(libc::SIGINT, handle_interrupt as libc::sighandler_t); // 2
    }

    std::thread::sleep(Duration::from_secs(10)); 
    println!("Goodbye");
}
```

当然，这个例子并不能满足要求信号处理机制的要求，因为`println!`显然调用stdio，而这并不异步信号安全。C中同样提供了逃脱异步信号安全函数限制的两个方式，`write`的管道，以及使用`signalfd`。在Rust中，这些方式同样可以使用libc库调用。



#### 自写管道

虽然很多函数都被认为是异步信号不安全，但是`write`并不在此列，意味着用户可以在信号处理函数中调用`write`和合适的文件描述符，在信号处理函数以及其他线程之间设置管道，从而传递信息。例如如下C例子，信号处理函数和主线程设置了管道，以便于主线程知晓何时信号处理器被调用：

```C
static int pipefds[2] = {0};

void signal_handler(int signum)
{
    uint8_t empty[1] = {0};
    int write_fd = pipefds[1];
    write(write_fd, empty, 1);                 // 3
}

void handle_signal(int read_pipe_fd)
{
    uint8_t buff[1] = {0};
    read(read_pipe_fd, buff, 1);               // 4
    printf("Received signal\n");
}

int main()
{
    pipe(pipefds)                              // 1
    fcntl(pipefds[1], F_SETFD, O_NONBLOCK);

    int read_fd = pipefds[0];

    signal(SIGINT, signal_handler);            // 2

    while(true) {
        handle_signal(read_fd);
    }
}
```



#### signalfd

除了使用`write`建立管道，[`signalfd`](https://man7.org/linux/man-pages/man2/signalfd.2.html)可以直接创建文件描述符来接受信号。使用`select`、`poll`、`epoll`等事件循环相关函数，用户可以非常轻松地处理文件描述符。例如：

```C
void handle_signal(int);

int main()
{
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);

    sigprocmask(SIG_SETMASK, &mask, NULL);                // 1

    int signal_fd = signalfd(-1, &mask, SFD_NONBLOCK);    // 2

    struct pollfd pollfd = {
        .fd = signal_fd,
        .events = POLLIN,
    };

    while (poll(&pollfd, 1, 5000) > 0)                    // 3
    {
        handle_signal(pollfd.fd);
    }
}

void handle_signal(int signal_fd)
{
    struct signalfd_siginfo siginfo;
    ssize_t s;
    s = read(signal_fd, &siginfo, sizeof(siginfo));       // 4
    if (s != sizeof(siginfo))
    {
        perror("read");
        exit(1);
    }

    uint32_t signo = siginfo.ssi_signo;
    char *signame = strsignal(signo);

    printf("Received signal %d (%s)\n", signo, signame);
}
```

在以上程序中，信号会被阻塞，写入信号文件描述符，并且按顺序等待被同步读。当事件循环告知主线程有信号待读时，信号处理函数被调用。通过信号文件描述符，信号处理函数可以获取同一个信号，并且处理信号。

信号文件描述符的使用类似于普通的代码，因此用户可以享有使用锁、信息序列、内存分配等等便利功能。不过，除此之外，信号文件描述符也有它的缺点，例如与子进程交互时可能会出现问题。子进程会继承父进程的信号阻塞设置，但是信号文件描述符并不会继承这个设置。因此，子进程要不不能正常接受信号，要不无法正常处理信号。当然，生成子进程时可以手动清楚继承的设置，并且想办法自己管理信号，但是这非常麻烦，甚至有人因此说[信号文件描述符毫无用处](https://ldpreload.com/blog/signalfd-is-useless)。因为这个原因，大部分子进程仍然使用自写管道而不事信号文件描述符来解决信号处理的问题。



### Rust: signal-hook

除了C FFI，Rust库signal-hook提供了信号处理需要的相应功能：

- 循环接受信号的迭代器，主线程可以从迭代器中获取下一个信号。
- 通过自写管道，主线程可从信号处理器中获取信号信息。
- 相应的适应器使得signal-hook也可以在tokio的事件循环和异步标准库中使用。

对于之前的C例子，如果使用signal-hook库改写，那么代码如下：

```rust
use signal_hook::consts::*;
use signal_hook::iterator::Signals;
use crossbeam::channel::{select, self, Sender, Receiver, after};
use std::time::Duration;

fn await_interrupt(interrupt_notification_channel: Sender<()>) {
    let mut signals = Signals::new(&[                              // 1：装载信号处理函数，形容libc::sigaction调用
        SIGINT,
    ]).unwrap();

    for s in &mut signals {                                        // 2：通过管道传递信号信息
        interrupt_notification_channel.send(());                   // 3：通过channel在线程间传递信息
    }
}

fn main() {
    let (interrupt_tx, interrupt_rx) = channel::unbounded();
    std::thread::spawn(move || { await_interrupt(interrupt_tx)});

    let timeout = after(Duration::from_secs(5));
    loop {
        select! {
            recv(interrupt_rx) -> _ => {                           // 4：当主线程接受到中断，循环将结束
                println!("Received interrupt notification");
                break;
            },
            recv(timeout) -> _ => {                                // 5：同时也有超时机制
                println!("Finally finished the long task");
                break;
            }
        }
    }
}
```



## 引用

特洛伊之源｜ 在 Rust 代码中隐藏无形的漏洞，![img](file:///C:\Users\98552\AppData\Roaming\Tencent\QQTempSys\8LDO48C$8@[GWU0353$FOVS.png)https://zhuanlan.zhihu.com/p/428305373

Working with signals in Rust - some things that signal handlers can't handle，https://www.jameselford.com/blog/working-with-signals-in-rust-pt1-whats-a-signal/

Unix信号，https://zh.wikipedia.org/wiki/Unix%E4%BF%A1%E5%8F%B7

Signal (IPC)，https://en.wikipedia.org/wiki/Signal_(IPC)

signal(7) — Linux manual page，https://man7.org/linux/man-pages/man7/signal.7.html

Linux 信号（signal），https://www.jianshu.com/p/f445bfeea40a

Interrupts，https://linux-kernel-labs.github.io/refs/heads/master/lectures/interrupts.html

简单介绍下linux下的中断（interrupt），https://blog.51cto.com/noican/1355357

Difference between Hardware Interrupt and Software Interrupt，https://www.geeksforgeeks.org/difference-between-hardware-interrupt-and-software-interrupt/

