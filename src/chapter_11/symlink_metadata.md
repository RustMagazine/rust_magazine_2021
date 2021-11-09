# 软链接引发的卡死问题

作者: 吴翱翔

> 原文: [软链接引发的卡死问题](https://pymongo.github.io/#/2021/11/symlink_metadata.md)

## fs::symlink_metadata

最近写的递归搜索文件夹统计大小的程序，测量某个文件夹大小时卡死

用 `find -type l` 去搜索软链接发现有两个链接循环引用:  
a 指向 b 的文件夹的同时 b 又指向 a 的文件夹

看源码才知 std::fs::metadata 底层调用 stat() 系统调用会 follow 链接

我改用 symlink_metadata 底层是 lstat() 的总算跳过软链接就解决掉卡死的 Bug

## errno 40 ELOOP

Linux glibc 的错误码 errno 40(ELOOP) 的大意是软链接跳转次数过多?

想着标准库会报错 ELOOP 后来野猫老师说 std::fs 为了跨平台做了很多牺牲

(好像 std::fs 的 API 不会处理 ELOOP 错误码? 但我从未见过 errno ELOOP 就不瞎说了)

## is_symlink() 永远返回 false?

标准库为了跨平台做出的 trade-off 例如 metadata() 的 is_syslink() 永远返回 false 显然对 Linux 系统而言设计的并不好

因为 metadata/stat 会"**吃掉**"软链接，相当于 *解析成* 普通文件也就是软链接的抽象就不存在了

所以 metadata().is_symlink() 在 Linux 上 **一定是永远返回 false**

所以 Rust 文档很贴心会强调 is_symlink() 要搭配 symlink_metadata() 一起使用才会生效

只有用 symlink_metadata/lstat 不追踪软链接的时候才能获知某个文件到底是不是软链接

当然 man 文档肯定也有提示，只不过我当初看的时候 TLDR 嫌太长没仔细看...

### is_symlink 与 is_dir 互斥

在 symlink_metadata()/lstat() 的返回值中

is_symlink() 和 is_dir() 互斥只能其中一个是 true 另一个是 false

## is_symlink 还没 stable 要怎么用?

不得不说标准库对各种 LinuxExt 的支持欠缺很多，is_symlink 预计要 2022 年初才能 stable

由于 Metadata 的成员字段全是私有的，只能 transmute 或者找找有没有 UnixExt 之类的

```rust
// 方法一: linux::fs::MetadataExt
let st_mode = std::os::linux::fs::MetadataExt::st_mode(&metadata);

// 方法二: transmute
let st_mode = unsafe { std::mem::transmute::<_, libc::mode_t>(metadata.file_type()) };

// 方法三: 我不用标准库了，直接调用 libc::stat 或 libc::lstat
```

## du 命令为啥跟 Metadata::len() 不一样

### 硬盘 4k 对齐

例如 a.txt 只有一个字符，stat 命令或 fs::Metadata::len() 去看确实大小为 1

但是用 du 去看却说 4k 的大小，原因是 Linux ext4 的文件系统的 block-size 一般是 4k

可以理解成硬盘的最小存储单位是 4k，所有文件占据硬盘的空间都是 4k 的整数倍，好像也叫 4k 对齐

有点像结构体内存布局要跟 CPU 寄存器大小 8 byte 对齐，结构体大小尽量要是 8 byte 的整数倍

如果 du 命令加上指定 block-size 的参数例如 `du --apparent-size --block-size 1` 就跟 stat 命令一样了

> `du --bytes` 或 `du -b` 是 `du --apparent-size --block-size 1` 的缩写

### /proc 真的是零大小吗

du 命令没骗你，`/dev`, `/proc`, `/sys` 这三个虚拟文件系统还真的在硬盘的大小为零(因为压根没存在硬盘上)

虽然 stat 命令去看这三文件夹绝大部分文件都是零大小，但是例如 "/proc/bus/pci/00/01.2" 还是有大小的

例如 "/proc/config.gz" 存的是 Linux kernel 的编译时参数

有很多参数的值都是 String 类型，所以说 zcat 出来的大小也是「变长」的或者说 **不确定的长度**

所以 stat 只是说 /proc/config.gz 在当前时刻如果读取出来那么长度会是多少而已

```
[w@ww repos]$ stat /proc/config.gz 
  File: /proc/config.gz
  Size: 58526           Blocks: 0          IO Block: 1024   regular file
Device: 0,21    Inode: 4026532079  Links: 1
Access: (0444/-r--r--r--)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 2021-11-08 21:11:50.742480314 +0800
Modify: 2021-11-08 21:11:50.742480314 +0800
Change: 2021-11-08 21:11:50.742480314 +0800
 Birth: -
[w@ww repos]$ du /proc/config.gz 
0       /proc/config.gz
```
