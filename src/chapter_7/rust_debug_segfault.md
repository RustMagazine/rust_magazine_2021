# Rust 如何调试内存错误及经验分享

作者: 吴翱翔 / 后期编辑: 张汉东

> 原文: [gdb/lldb 调试 segfault](https://pymongo.github.io/#/2021/06/custom_rust_lint.md)

## segfault 的问题代码

以下是我重写 ls 命令的部分源码(以下简称`ls 应用`)，完整源码在[这个代码仓库](https://github.com/pymongo/linux_commands_rewritten_in_rust/blob/19055db6fae6a22ffb219f9bf0d1107f6d2db917/src/bin/ls.rs#L12-L26)

```rust
fn main() {
    let dir = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
    loop {
        let dir_entry = unsafe { libc::readdir(dir) };
        if dir_entry.is_null() {
            break;
        }
        // ...
    }
}
```

当 ls 应用没有任何参数时，默认会把当前文件夹作为参数，以上代码当参数是一个文件夹时能正常运行

```
> cargo r --bin ls
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ls`
Cargo.toml
Cargo.lock
..
.
src
target
.gitignore
.git
```

但是当 ls 应用的参数不是文件夹时，就会 segfault 内存段错误:

```
> cargo r --bin ls -- Cargo.toml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ls Cargo.toml`
Segmentation fault (core dumped)
```

## coredumpctl 查看错误

首先查看系统配置文件 `/proc/config.gz` 看看是否已开启 coredump 记录功能

```
> zcat /proc/config.gz | grep CONFIG_COREDUMP
CONFIG_COREDUMP=y
```

由于 `/proc/config.gz` 是二进制格式而非文本格式，所以要用 `zcat` 而非 `cat` 去打印

> coredumpctl list

通过 `coredumpctl list` 找到最后一条 coredump 记录，也就是刚刚发生的 segfault 错误记录

> Tue 2021-07-06 11:20:43 CST 358976 1000 1001 SIGSEGV present  /home/w/repos/my_repos/linux_commands_rewritten_in_rust/target/debug/ls  30.6K

注意用户 id 1000 前面的 358976 表示进程的 PID，用作 `coredumpctl info` 查询

> coredumpctl info 358976

```
           PID: 358976 (segfault_opendi)
           UID: 1000 (w)
           GID: 1001 (w)
        Signal: 11 (SEGV)
     Timestamp: Tue 2021-07-06 11:20:43 CST (3min 13s ago)
  Command Line: ./target/debug/ls
    Executable: /home/w/repos/my_repos/linux_commands_rewritten_in_rust/target/debug/ls
 Control Group: /user.slice/user-1000.slice/user@1000.service/app.slice/app-org.kde.konsole-8d381e4d42bf46bbabb81e9b03d5be1f.scope
          Unit: user@1000.service
     User Unit: app-org.kde.konsole-8d381e4d42bf46bbabb81e9b03d5be1f.scope
         Slice: user-1000.slice
     Owner UID: 1000 (w)
       Boot ID: d464328302f146f99ed984edc6503ca0
    Machine ID: 84d31ba0e3154ceb82a12fcbc8be2625
      Hostname: ww
       Storage: /var/lib/systemd/coredump/core.segfault_opendi.1000.d464328302f146f99ed984edc6503ca0.358976.1625541643000000.zst (present)
     Disk Size: 30.6K
       Message: Process 358976 (segfault_opendi) of user 1000 dumped core.
                
                Stack trace of thread 358976:
                #0  0x00007f0284c9c904 readdir (libc.so.6 + 0xc7904)
                #1  0x00005559c451964e n/a (/home/w/repos/my_repos/linux_commands_rewritten_in_rust/target/debug/ls + 0x1364e)
```

也可以选择用 gdb 解析 segfault 的 coredump 文件:

> coredumpctl gdb 358976

或者

> coredumpctl debug 358976

参考: [core dump - wiki](https://wiki.archlinux.org/title/Core_dump)

## dmesg 查看 segfault

`sudo dmesg` 能查看最近几十条内核消息，发生 segfault 后能看到这样的消息:

> [73815.701427] ls[165042]: segfault at 4 ip 00007fafe9bb5904 sp 00007ffd78ff8510 error 6 in libc-2.33.so[7fafe9b14000+14b000]

所以从 dmesg 内核消息中可以发现 segfault 发生在「libc-2.33.so」库文件中

## valgrind 检查内存错误

> gdb ./target/debug/ls

```
// ...
==356638== Process terminating with default action of signal 11 (SIGSEGV): dumping core
==356638==  Access not within mapped region at address 0x4
==356638==    at 0x497D904: readdir (in /usr/lib/libc-2.33.so)
==356638==    by 0x11B64D: ls::main (ls.rs:15)
// ...
```

相比 dmesg 只能看到在哪个库段错误了, valugrind 还能知道 Rust 代码第几行出错了

但是 valgrind 无法获知报错时各个变量的值进而发现错误原因，所以还是需要 gdb/lldb 调试

## gdb 调试

§ gdb 打开 ls 应用的可执行文件:

> valgrind --leak-check=full ./target/debug/ls

§ gdb 通过 `l` 或 `list` 命令打印可执行文件的代码:

> (gdb) l

§ gdb运行 ls 应用且传入 `Cargo.toml` 文件名作为入参:

> (gdb) run Cargo.toml

```
Starting program: /home/w/repos/my_repos/linux_commands_rewritten_in_rust/target/debug/ls Cargo.toml
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x00007ffff7e5a904 in readdir64 () from /usr/lib/libc.so.6
```

§ 查看 segfault 发生时的栈帧

使用 gdb `where` 或 `bt` 命令打印 backtrace

> (gdb) where

```
#0  0x00007ffff7e5a904 in readdir64 () from /usr/lib/libc.so.6
#1  0x0000555555568952 in ls::main () at src/bin/ls.rs:15
```

此时已经找到出问题的系统调用函数是 `readdir64`，而且是在 `ls.rs` 的 14 行调用的 `readdir`

§ 查看 ls::main 栈帧的局部变量

- `info variables` 能打印全局或 static 变量
- `info locals` 打印当前栈帧的局部变量
- `info args` 打印当前栈帧的入参

> (gdb) frame 1
> 
> (gdb) info locals

```
(gdb) frame 1
#1  0x0000555555569317 in ls::main () at src/bin/ls.rs:20
20              let dir_entry = unsafe { libc::readdir(dir) };
(gdb) info locals
dir = 0x0
// ...
```

此时发现 main 栈帧的 `dir = 0x0` 是空指针，导致 readdir 系统调用 segfault

参考 gnu.org 的官方教程: <https://www.gnu.org/software/gcc/bugs/segfault.html>

## lldb 调试

lldb 调试和运行可执行文件的几乎一样，主要就 backtrace 的命令和打印内容不同

§ lldb 通过 `thread backtrace` 打印 backtrace，跟 gdb 的 where/bt 命令略有不同

> (lldb) thread backtrace 

```
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
* thread #1, name = 'ls', stop reason = signal SIGSEGV: invalid address (fault address: 0x4)
  * frame #0: 0x00007ffff7e5a904 libc.so.6`readdir + 52
    frame #1: 0x0000555555568952 ls`ls::main::h5885f3e1b9feb06f at ls.rs:15:34
    frame #2: 0x000055555556922b ls`core::ops::function::FnOnce::call_once::hca3142f69818a896((null)=(ls`ls::main::h5885f3e1b9feb06f at ls.rs:4), (null)=<unavailable>) at function.rs:227:5
    frame #3: 0x000055555556ca5e ls`std::sys_common::backtrace::__rust_begin_short_backtrace::hf7701799a2595181(f=(ls`ls::main::h5885f3e1b9feb06f at ls.rs:4)) at backtrace.rs:125:18
// ...
```

§ lldb 通过 `frame select n` 切换当前栈帧，而 gdb 则是 `frame n`

> (lldb) frame select 1

```
frame #1: 0x0000555555569317 ls`ls::main::h5885f3e1b9feb06f at ls.rs:15:34
   12       
   13       let dir = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
   14       loop {
-> 15           let dir_entry = unsafe { libc::readdir(dir) };
   16           if dir_entry.is_null() {
   17               // directory_entries iterator end
   18               break;
```

lldb 的选中栈帧时还能打印附近几行代码，这点比 gdb 只显示一行代码好多了

§ lldb `frame variable` 等于 gdb 的 `info args` 加上 `info locals`

`(gdb) info args` 等于 `(lldb) frame variable --no-args`

> (lldb) frame variable

除了 primitive types, lldb 还可以打印 String 类型变量的值，但是无法得知 `Vec<String>` 类型变量的值

但 gdb 只能打印 primitive types 的数值，可能这也是 Intellij-Rust 和 vscode 默认用 lldb 调试 Rust 代码的原因

由于 coredumpctl 只能用 gdb 调试 coredump 文件，因此开发人员必须熟练掌握 gdb

## vscode-lldb 调试

rust-analyzer 插件不打任何断点，点 Debug 运行 会定位到 readdir 系统调用函数的反汇编代码中:

> 7FFFF7E5A904: 0F B1 57 04                cmpxchgl %edx, 0x4(%rdi)

此时应当关注 vscode 左侧 Debug 侧边栏的 `CALL STACK` 菜单 (lldb thread backtrace )

call stack 菜单会告诉 readdir 当前汇编代码的上一帧(也就是 backtrace 第二个栈帧)是 main 函数的 15 行

点击 main 栈帧就能跳转到出问题的源码所在行了

在 main 栈帧 下再通过 variable 菜单发现 readdir 传入的 dir 变量值为 NULL 导致段错误

## Intellij-Rust 调试

Debug 运行直接能跳转到问题代码的所在行，并提示 `libc::readdir(dir)` 的 dir 变量的值为 NULL

---

## 分析错误原因

```rust
let dir = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
loop {
    let dir_entry = unsafe { libc::readdir(dir) };
    if dir_entry.is_null() {
        break;
    }
    // ...
}
```

问题出在没有判断 `opendir` 系统调用是否成功，系统调用失败要么返回 NULL 要么返回 -1

如果 `opendir` 系统调用传入的文件类型不是 directory，就会调用失败

## 解决 segfault

只需要在加上 dir 是否为 NULL 的代码，如果为 NULL 则打印系统调用的错误信息

```rust
if dir.is_null() {
    unsafe { libc::perror(input_filename.as_ptr().cast()); }
    return;
}
```

再次测试 ls 应用读取非文件夹类型的文件

```
> cargo r --bin ls -- Cargo.toml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ls Cargo.toml`
Cargo.toml: Not a directory
```

此时程序没有发生段错误，并且打印了错误信息

再试试让 ls 应用处理一个正常的文件夹类型的文件，输出也正常

关于修复 ls 应用 segfault 的代码改动[在这个 commit](https://github.com/pymongo/linux_commands_rewritten_in_rust/commit/b5f92f85ab1949e04ac713ad079d4359760e1cd1)

## SIGABRT 调试案例分享

以下是一段深度优先搜索遍历文件夹的代码(省略部分无关代码，[完整源码链接在这](https://github.com/pymongo/linux_commands_rewritten_in_rust/blob/main/examples/sigabrt_closedir_wrong.rs))

```rust
unsafe fn traverse_dir_dfs(dirp: *mut libc::DIR, indent: usize) {
    loop {
        let dir_entry = libc::readdir(dirp);
        if dir_entry.is_null() {
            let _sigabrt_line = std::env::current_dir().unwrap();
            return;
        }
        // ...
        if is_dir {
            let dirp_inner_dir = libc::opendir(filename_cstr);
            libc::chdir(filename_cstr);
            traverse_dir_dfs(dirp_inner_dir, indent + 4);
            libc::chdir("..\0".as_ptr().cast());
            libc::closedir(dirp);
        }
    }
}
```

这段代码运行时会报错:

```
malloc(): unsorted double linked list corrupted

Process finished with exit code 134 (interrupted by signal 6: SIGABRT)
```

由于 C 的函数没法应用所有权等静态分析，所以非常考验开发人员的**经验**

通过 gdb 调试能知道 `std::env::current_dir()` 调用报错了，但错误原因未知

### 经验: SIGABRT 可能原因

通过上述段错误的分析，我们知道 SIGSEGV 可能的原因是例如 `readdir(NULL)` 解引用空指针

根据作者开发经验，SIGABRT 的可能原因是 **double free**

### valgrind 检查 double free

顺着 double-free 的思路，通过 valgrind 内存检查发现，`libc::closedir(dirp)` 出现 InvalidFree/DoubleFree 的内存问题

### 分析 double free 原因

再细看源码，递归调用前创建的是子文件夹的指针，递归回溯时却把当前文件夹指针给 close 掉了

这就意味着，一旦某个目录有 2 个以上的子文件夹，那么当前的文件夹指针可能会被 free 两次

进而将问题的规模简化成成以下三行代码:

```rust
let dirp = libc::opendir("/home\0".as_ptr().cast());
libc::closedir(dirp);
libc::closedir(dirp);
```

### double free 的通用解决方法

C 编程习惯: free 某个指针后必须把指针设为 NULL

```rust
let mut dirp = libc::opendir("/home\0".as_ptr().cast());
libc::closedir(dirp);
dirp = std::ptr::null_mut();
libc::closedir(dirp);
dirp = std::ptr::null_mut();
```

在「单线程应用」中，这种解决方法是可行的，

第一次 free 后 dirp 指针被成 NULL，第二次 free 时传入 dirp 则什么事都不会发生

因为大部分的 C/Java 函数第一行都会判断输入是否空指针 `if (ptr == null) return`

### 为什么有时 double free 没报错

有个问题困惑了我: 
- 为什么连续写几行 closedir 进程会提前 SIGABRT?
- 为什么循环中多次 closedir 进程还能正常退出?
- 为什么循环中多次 closedir 调用 `std::env::current_dir()` 时就 SIGABRT?

原因是 double free 不一定能及时被发现，所以程序可能报错 SIGABRT 也可能正常退出

尤其是我这案例代码中多个递归循环之间隐式的资源共享问题导致 double free 可能就不会报错

进程 free 掉的堆内存可能不会立即被操作系统回收，

由于 `current_dir` 函数内需要构建动态的 buffer 以及动态扩容，

所以会频繁向操作系统申请堆内存，这时候 Rust 内存分配器发现有 double free 不合法的内存就中止了进程

可以将 double free 问题比喻成异步的 Future，

虽然有些堆内存 double free 异常掉了，但操作系统回收或检测 double free 可能是异步的，不会及时报错 SIGABRT

但这时 Rust 的内存分配器突然要检查堆内存，就像 Future 的 poll，才终于把之前的内存问题给暴露出来

由于作者对操作系统理解有限，以上对 double free 不会及时报错纯属个人看法，难免有错误或纰漏，望读者理解

## 如何学习更多 Rust 调试经验

可以作者这样把 Linux 所有命令都用 Rust 写一遍

项目链接: <https://github.com/pymongo/linux_commands_rewritten_in_rust/>

目前作者用 Rust 写完的 Linux 命令有:
- chmod
- id
- ls
- pwd
- stat
- tree

从零用 Rust 写一遍所有 Linux 命令，基本什么样的内存错误都能遇得到，积累更多 Rust 调试经验
