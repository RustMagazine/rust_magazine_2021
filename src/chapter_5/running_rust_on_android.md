# 在 Android 中运行 Rust 

> [原文](https://blog.svgames.pl/article/running-rust-on-android) / 译者：iamazy

为了我目前的一位客户，我们决定将 Rust 作为我们主要的编程语言。做出这个决定的原因有很多：除了技术优势外，还有一个无可争议的事实就是：Rust 仍然是一门相对较新的语言，花哨且时髦 - 当你是一家初创公司时，使用十几年前的技术可能会让你陷入困境。我的意思是 - 不使用创新的技术如何进行创新？最快的成功方式就是对其大肆宣传。

”用户持有自己的数据“应该是产品的一个卖点，它不能是一个完全通过浏览器访问的服务，而应该是一种可以分发给用户，并让其运行在用户设备上的某个东西。我们在内部已经运行了一些 headless 实例，只要再完成一些琐碎的工作，就可以为 Windows 和 Linux 系统制作可重新分发的程序包。但是我们知道如果程序包只能运行在桌面操作系统中时，将会严重阻碍应用的普及 - 如果我们想让它脱颖而出，则需要该应用程序的移动版本。这意味着我们必须要知道如何让我们的程序运行在 Android 或者 iOS 系统中。因为我对交叉编译与自动化构建已经有了一些经验，因此我主动的研究了这个主题。

## 获取工具

先从基础开始，我需要获取 Rust 交叉编译器。幸运的是，Rust 让此操作变得十分简单，因为只需要调用以下命令：

```console
$ rustup target add armv7-linux-androideabi  # For 32-bit ARM.
$ rustup target add aarch64-linux-android    # For 64-bit ARM.

# x86_64 is mainly useful for running your app in the emulator.
# Speaking of hardware, there are some commercial x86-based tablets,
# and there's also hobbyists running Android-x86 on their laptops.
$ rustup target add x86_64-linux-android
```
*（注意：以后只会显示 aarch64 架构的所有示例）*

我还需要 Android 的构建工具。在经过一番调研之后，我来到 [Android Studio 的下载页面](https://developer.android.com/studio#command-tools) 并抓取了归档的命令行工具。尽管 SDK 包有 80+ MiB 大小，却依然只有所需工具的最小子集，所以我听从了互联网的建议并使用`sdkmanager`来安装额外的部件。

```console
$ cd ~/android/sdk/cmdline-tools/bin/
$ ./sdkmanager --sdk_root="${HOME}/android/sdk" --install 'build-tools;29.0.2'
$ ./sdkmanager --sdk_root="${HOME}/android/sdk" --install 'cmdline-tools;latest'
$ ./sdkmanager --sdk_root="${HOME}/android/sdk" --install 'platform-tools'
$ ./sdkmanager --sdk_root="${HOME}/android/sdk" --install 'platforms;android-29'
```

尽管 Android 支持运行 native 代码，但是大多数应用还是采用 Java 或者 Kotlin 来编写，SDK 反应了这一点。为了能够使用 native 代码，我还需要一个工具 - 原生开发工具套件 (Native Development kit)。[NDK 下载页面](https://developer.android.com/ndk/downloads) 提供了几个版本以供选择 - 在经过一段深思熟虑后，我决定使用 LTS 版本：r21e。

## 足够简单！或想太多？

搞定了开发工具之后，我决定试着直接编译项目。

```console
$ cargo build --target=aarch64-linux-android
```

和预期的一样，构建失败了，并且错误信息占满了整个屏幕。经过筛选后，显示存在一个链接错误：

```console
error: linking with `cc` failed: exit code: 1
/usr/bin/ld: startup.48656c6c6f20546865726521.o: Relocations in generic ELF (EM: 183)
/usr/bin/ld: startup.48656c6c6f20546865726521.o: error adding symbols: file in wrong format
collect2: error: ld returned 1 exit status
```

我认为这(错误提示)足够简单 - Cargo 试图使用系统的链接器而不是 Android NDK 的链接器。我可以使用`CC`和`LD`环境变量让 Cargo 指向正确的链接器。

```console
$ export ANDROID_NDK_ROOT="${HOME}/android/ndk"
$ export TOOLCHAIN="${ANDROID_NDK_ROOT}/toolchains/llvm/prebuilt/linux-x86_64"
$ export CC="${TOOLCHAIN}/bin/aarch64-linux-android29-clang"
$ export LD="${TOOLCHAIN}/bin/aarch64-linux-android-ld"
$ cargo build --target=aarch64-linux-android
```

让我失望的是，这并没有起作用。我不愿意花费一天的时间来和 Cargo 纠缠，因此我决定寻找是否有其他人给出了解决方案 - 很快，我便找到看似十分完美的工具。

## cargo-apk

[cargo-apk](https://crates.io/crates/cargo-apk) 是一个可以简单的将 Cargo 项目构建成`.apk`的工具。你所需要做得就是安装这个工具，在`Cargo.toml`文件中添加一些配置，然后你就可以继续了。

```toml
# cargo-apk compiles your code to an .so file,
# which is then loaded by the Android runtime
[lib]
path = "src/main.rs"
crate-type = ["cdylib"]
 
# Android-specic configuration follows.
[package.metadata.android]
# Name of your APK as shown in the app drawer and in the app switcher
apk_label = "Hip Startup"
 
# The target Android API level.
target_sdk_version = 29
min_sdk_version = 26
 
# See: https://developer.android.com/guide/topics/manifest/activity-element#screen
orientation = "portrait"
```

有了上面添加的配置，我试图使用`cargo-apk`来构建项目。

```console
$ cargo install cargo-apk
$ export ANDROID_SDK_ROOT="${HOME}/android/sdk"
$ export ANDROID_NDK_ROOT="${HOME}/android/ndk"
$ cargo apk build --target aarch64-linux-android
```

令人惊奇的是，它成功了！(等等) 额，好吧，我再一次遇到了链接错误。但是这一次，它不是关于重定位和文件格式的神秘错误，而是一个缺少链接库的错误提示：

```console
error: linking with `aarch64-linux-android29-clang` failed: exit code: 1
    aarch64-linux-android/bin/ld: cannot find -lsqlite3
    clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

## 依赖，依赖，依赖

我们的项目使用 [SQLite](https://sqlite.org/)，这是一个 C 库。尽管 Rust 社区在每个可能的场合都吹捧”用 Rust 重写“在某种程度上是臭名昭著的，但是实际上某些与流行库一起使用的 crate 并不需要重新实现，因为这需要大量的工作。相反，它们仅提供在 Rust 代码中调用库的方式，既可以作为 C 函数重新导出，也可以提供更加友好的 API 并稍微抽象化 FFI 调用。我们使用的 [rusqlite](https://crates.io/crates/rusqlite) 并没有什么不同，意味着我们也需要构建 SQLite。

SQLite 使用 GNU Autotool 进行构建。在对环境变量和用于配置的选项有了一些了解之后，我仔细浏览了 NDK 的文档 - 我找到了一个在各种构建系统([包括 Autotools](https://developer.android.com/ndk/guides/other_build_systems#autoconf)) 中使用 NDK 的文档页面。尽管 Google 提供了 LTS 版本的 NDK，以及最新版本的文档，但在 r21 LTS 和最新的 r22 之间发生了变化，事情变得稍微有点棘手。幸运的是，Wayback 机器具有该页面的[历史版本](http://web.archive.org/web/20200531051836/https://developer.android.com/ndk/guides/other_build_systems#autoconf)，让我能够找到合适的 NDK r21 的说明。

```console
$ ANDROID_API=29
$ TOOLCHAIN="${ANDROID_NDK_ROOT}/toolchains/llvm/prebuilt/linux-x86_64"i
$ export CC="${TOOLCHAIN}/bin/aarch64-linux-android${ANDROID_API}-clang"
$ export CXX="${TOOLCHAIN}/bin/aarch64-linux-android${ANDROID_API}-clang++"
$ export AR="${TOOLCHAIN}/bin/aarch64-linux-android-ar"
$ export AS="${TOOLCHAIN}/bin/aarch64-linux-android-as"
$ export LD="${TOOLCHAIN}/bin/aarch64-linux-android-ld"
$ export RANLIB="${TOOLCHAIN}/bin/aarch64-linux-android-ranlib"
$ export STRIP="${TOOLCHAIN}/bin/aarch64-linux-android-strip"
$ ./configure --host=aarch64-linux-android --with-pic
$ make -j $(nproc)
```

## Pick me up, Scotty

使用上述方法，成功构建了 SQLite，生成了`libsqlite3.so`。现在只需要知道如何让 Cargo 使用它即可。在浏览 Cargo Book 时，我遇到了讲述[环境变量](https://doc.rust-lang.org/cargo/reference/environment-variables.html)的一个章节，它提及了`RUSTFLAGS`。和 Make 或 CMake 对待`CFLAGS`和`CXXFLAGS`一样，`RUSTFLAGS`的内容被 Cargo 传递给`rustc`编译器，允许它影响编译器的行为。

尽管这种方式十分简单，但是对我来说不是很优雅，因此我进一步深入研究了其他选项。继续浏览 Cargo Book，我遇到了描述项目配置的章节，可以肯定的是，[有一种方法可以指定 RUSTFLAGS](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)。然而，无论我如何尝试，我始终都会收到来自 Cargo 的提示，告诉我关于未使用的 manifest 键的信息。

```console
warning: unused manifest key: target.aarch64-linux-android.rustflags
```

浏览 Cargo Book 的更多章节，我遇到了关于[构建脚本](https://doc.rust-lang.org/cargo/reference/build-scripts.html)的章节。它们毫无疑问是一个强大的工具，但是我已经花费了很多时间学习 Cargo 的配置，不想再花更多的时间阅读关于如何编写构建脚本的内容，因此，最终我选择了环境变量的解决方案，~~并且可能会在之后尝试使用构建脚本的方式~~(不可能)。

我在终端中输入命令，并焦急的观察它的执行过程。

```console
$ RUSTFLAGS="-L $(pwd)/sqlite-autoconf-3340000/.libs/" cargo apk build --target aarch64-linux-android
```

再一次，它。。。在某种程度上成功了。虽然链接器不再将错误解释成缺失链接库，但是`cargo-apk`无法找到该链接器并将其添加到最终的 APK 文件中。

```console
 'lib/arm64-v8a/libstartup.so'...
Shared library "libsqlite3.so" not found.
Verifying alignment of target/debug/apk/statup.apk (4)...
      49 AndroidManifest.xml (OK - compressed)
     997 lib/arm64-v8a/libstartup.so (OK - compressed)
Verification succesful
```

当我还没有编译`libsqlite3.so`时，我返回上一步仔细阅读了链接器产生的错误信息。链接器组合了很多的目标文件，这些目标文件都位于`target/aarch64-linux-android/debug/deps`目录下。如果我将`.so`文件放在这里会发生什么？

```console
$ cp sqlite-autoconf-3340000/.libs/sqlite3.so target/aarch64-linux-android/debug/deps
$ cargo apk build --target aarch64-linux-android
```

让我惊讶的是，它成功了！

```console
 'lib/arm64-v8a/libstartup.so'...
 'lib/arm64-v8a/libsqlite3.so'...
Verifying alignment of target/debug/apk/startup.apk (4)...
      49 AndroidManifest.xml (OK - compressed)
     997 lib/arm64-v8a/libstatup.so (OK - compressed)
15881608 lib/arm64-v8a/libsqlite3.so (OK - compressed)
Verification succesful
```

我现在有了一个可以安装在 Android 手机上的`.apk`文件。真是个巨大的成功！

## 应用 和 Activity

将 Rust 代码编译进`.apk`中后，我们剩下要做的就是要搞清楚如何将 Rust 代码与编写 UI 的 Java 代码合并。我天真的在 DuckDuckGo 中输入“如何组合 APK”。在阅读顶层几个结果后，明白了这明显是不可能的，至少在对 Android 应用的工作原理没有更深的了解的情况下是不可能的。但是，并不是说没有其他的方法，因为文章提出了另一种方法 - 将 [Activity](https://developer.android.com/reference/android/app/Activity) 组合到一个应用程序里。

如果你像我一样，之前从未开发过 Android，可能会疑惑“什么是 Activity”：当你设计一个应用时，它就是所谓的“界面”或者“视图”。例如，在购物应用中：

- 登陆页面是一个 Activity
- 产品搜索页面是一个 Activity
- 所选产品的详情页面是一个 Activity
- 购物车页面是一个 Activity
- 结账页面是一个 Activity

这里的每个页面可能都包含一些交互元素，如无处不在的汉堡包菜单。如果你愿意，从理论上来讲，你可以将整个应用程序放在一个单独的 Activity 中，但是开发难度比较大。当然，关于 Activity 还有很多内容可以介绍，但是目前和我们要讲的内容关系不大。

让我们继续介绍有关 Rust 的内容。虽然我的问题的解决方案是将 Activity 组合到一个应用程序中，但是我不确定用 Rust 构建的`.apk`文件是如何与所有这些联系在一起的。在仔细研究了 [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/blob/b430a5e274dea8fd7c45e176d5d19c31b73a20ac/ndk-glue/src/lib.rs#L132) 代码之后，我意识到它本质是将我的代码封装进一些胶水代码中，并为 Android 的运行创建 [NativeActivity](https://developer.android.com/reference/android/app/NativeActivity)。

为了将 Activity 组合进一个应用中，我需要修改应用程序的`AndroidManifest.xml`文件，在文档中添加合适的 [Activity 节点](https://developer.android.com/guide/topics/manifest/activity-element)。但是我应该如何知道`cargo-apk`生成的 NativeActivity 的属性呢？幸运的是，当`cargo-apk`工作时，它会生成一个最小版的`AndroidManifest.xml`文件，并将其放在生成的`.apk`旁边。其中 NativeActivity 的声明如下所示：

```xml
<activity
    android:name="android.app.NativeActivity"
    android:label="startup"
    android:screenOrientation="portrait"
    android:launchMode="standard"
    android:configChanges="orientation|keyboardHidden|screenSize">
    <meta-data android:name="android.app.lib_name" android:value="startup" />
    <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
    </intent-filter>
</activity>
```

我要做的就是将上面的代码片段复制并粘贴到 Java 应用程序的 manifest 中。

当然，这只是在应用的 manifest 中添加了一条语句，告诉应用将要包含哪些 Activity。Java 应用程序的构建过程不会知道`libstartup.so`文件的位置，并自动的将其包含在内。幸运的是，我只需要将[库文件复制到指定的文件夹下](https://developer.android.com/studio/projects/gradle-external-native-builds#jniLibs)即可，Gradle (Android 应用的构建工具) 会自动将它们采集起来。

```console
$ mkdir -p android/app/src/main/jniLibs/arm64-v8a
$ cp sqlite-autoconf-3340000/.libs/libsqlite3.so android/app/src/main/jniLibs/arm64-v8a/
$ cp target/aarch64-linux-android/debug/libstatup.so android/app/src/main/jniLibs/arm64-v8a/
$ cd android/ && ./gradlew && ./gradlew build
```

这些都完成后，我启动了构建，它成功了！我将`.apk`安装在我闲置的 Android 设备中，但是...好像有哪里不太对劲呢！

![two-launcher-activities](./image/two-launcher-activities.png)

我的应用一旦安装成功后，会在应用的启动界面产生两个快捷方式。其中一个启动 Java 的 UI 界面，而另一个启动包含 Rust 代码的 NativeActivity。在阅读了更多关于 Activity 和 AndroidManifest 的内容后，我了解到，造成此问题的部分是 NativeActivity 的 [<intent-filter>](https://developer.android.com/guide/topics/manifest/intent-filter-element) - 即 [category](https://developer.android.com/reference/android/content/Intent#CATEGORY_LAUNCHER) 节点声明应在启动器中显示它。一旦我将它移除，一切就会恢复正常，NativeActivity 不再显示在启动器中。

但是，仍然存在一个问题：我如何让 Java 的 Activity 要求 Rust 的 Activity 为其工作？

## 恶意的 Intent

Android 中的 Activity 可以毫无问题的相互启动 - 如果这不可能，则无法真正在两者之间传递用户信息。调用另一个 Activity 的标准方法是通过 [startActivity()](https://developer.android.com/reference/android/app/Activity#starting-activities-and-getting-results) 方法，该方法接收一个参数：[Intent](https://developer.android.com/reference/android/content/Intent.html) 类实例。

尽管 Intent 类的名称是不言而喻的，但是起初它的用法可能有点不直观。在它最基本的形式中，它仅包含对调用 Activity 实例的引用，以及我们要调用的 Activity 的类句柄。(确切的说，一个 Intent 需要调用一个 [Context](https://developer.android.com/reference/android/content/Context.html)。Activity 只是 Context 的一种类型)。

但是，Intent 也可以用于传达为什么一个 Activity 会调用另一个 Activity 的信息(例如 [action](https://developer.android.com/reference/android/content/Intent#standard-activity-actions))，可以用来区分例如“显示某些内容”和“编辑某些内容”；或要操作的数据 URI 及其 MIME 类型。除了 get/set 方法，Intent 还可以容纳几乎任何数量的“额外”数据，这些数据通常作为键值对存储。

Intent 提供了一种在 Activity 之间传递信息的标准化方式。调用者向被调用者提供处理其请求所需的一切信息，并且它可以接收包含所有请求信息的另一个 Intent 作为返回值。使用 Java 编写代码时，没有什么问题，但是，将 Rust 代码放入 NativeActivity 会发生什么？

如果你查看继承树，你可以看到 NativeActivity 继承了 Activity - 这意味着它可以访问 Activity 所有非私有方法。我可以调用`getIntent()`并从调用者中获取数据。除此之外，由于这是 Java 方法，并且我是在 native 代码中运行，因此需要使用 JNI (Java Native Interface) 执行函数调用。不幸的是，NativeActivity 没有任何其他的机制来传递信息或使用 Intent。这让我十分沮丧，因为这意味着我必须要与 JNI 一起工作。

## JNI 之旅

在这一点上，我花了太多时间却没有取得明显的成果，这让我感到十分沮丧。另一方面，我意识到使用 JNI 带来了一些新的可能 - 不必使用 Activity 和 Intent，我可以将代码粘贴在函数中，并通过调用参数和返回值进行通信。有了这个新思路，我开始了对 JNI 的研究。

因为在 Java 中，万物皆对象，并且代码不能存在于类之外的部分 - native 代码也必须是类的一部分。因为我不需要持久化，因此使用静态方法即可。

```java
package com.startup.hip;
 
public class RustCode {
    public static native void doStuff();
}
```

上面是一个 Java 类的最小示例，其中带有一个标记为`native`的静态方法。有了这个，我需要实现相应的功能。但是我应该如何正确的使用函数签名呢？

幸运的是，Java 具有为 JNI 生成 C 语言头文件的功能。在 Java SE9 之前，它是一个独立的工具 - [javah](https://docs.oracle.com/javase/9/tools/javah.htm)；后来，它作为`-h`选项合并到了主要的`javac`编译器可执行文件中。该选项需要一个目录参数，用来放置生成的`.h`文件。用法十分简单。

```console
$ javac -h ./ RustCode.java
```

调用上面的命令将创建一个`com_startup_hip_RustCode.h`文件，其中包含函数定义。

```cpp
#include <jni.h>
JNIEXPORT void JNICALL Java_com_startup_hip_RustCode_doStuff(JNIEnv *, jclass);
```

有了这些知识，我就可以继续在 Rust 中创建适当的函数了。

## C++ 闪回

当处理外部代码时，Rust 和 C 很像，主要是使用 [extern 块](https://doc.rust-lang.org/reference/items/external-blocks.html)。此外，与 C++ 一样，Rust 可以使用 [name mangling](https://en.wikipedia.org/wiki/Name_mangling) - 这不足为奇，因为这门语言对范型和宏提供了强大的支持。幸运的是，Rust 提供了一种简单的方式来禁用 name mangling - 使用 [#[no mangle]](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-rust-functions-from-other-languages) 注解。

```rust
use jni::{objects::JClass, JNIEnv};
 
#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff(
    _env: JNIEnv,
    _class: JClass,
) {}
```

创建了函数声明之后，接下来我需要编写对应的实现。

## 接收参数

通常，native 函数需要接收一些参数。在本例中，它是一个包含代码的字符串，该代码随后将被传递给服务端。

```java
package com.startup.hip;
 
public class RustCode {
    public static native void doStuff(String code);
}
```

修改 Java 代码之后，我重新生成了 C 语言的头文件并据此编辑了 Rust 代码。

```rust
use jni::{objects::JClass, JNIEnv};
 
#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff(
    _env: JNIEnv,
    _class: JClass,
    code: JString,
) {}
```

这很简单。现在我需要从 Java 字符串中提取文本并将其传递给 Rust 代码。这比我预期要复杂的多。问题在于，JVM 内部使用 [UTF-8 的修改版本](https://docs.oracle.com/en/java/javase/11/docs/specs/jni/types.html#modified-utf-8-strings)存储字符串，而 Rust 字符串必须是有效的 [UTF-8](https://doc.rust-lang.org/std/string/struct.String.html#utf-8)。尽管 Rust 具有用于[处理任意字符串](https://doc.rust-lang.org/std/ffi/struct.OsString.html)的类型，但是我们的代码仅使用“经典”的字符串类型，对其进行全部修改需要大量工作。

幸运的是，`jni`库带有内置的机制，可以通过特殊的 [JNIStr](https://docs.rs/jni/0.19.0/jni/strings/struct.JNIStr.html) 类型在标准 UTF-8 和 JVM 修改后的 UTF-8 之间进行转换。在仔细阅读了文档之后，我想到了以下代码：

```rust
// Convert from JString – a thinly wrapped JObject – to a JavaStr
let code_jvm = env.get_string(code).unwrap();
// Create a String from JavaStr, causing text conversion
let code_rust = String::from(code_jvm);
```

现在我有了一个 Rust 字符串，可以将其传递给之后的 Rust 代码。又是个巨大的成功！

## 返回值

接收参数只是故事的一半，我还需要一个返回值，巧的是，它也是一个字符串 - 一个代表服务端返回值的字符串。

```java
package com.startup.hip;
 
public class RustCode {
    public static native String doStuff(String code);
}
```

我再一次修改了 Java 代码，重新生成了 C 语言的头文件，并据此编辑 Rust 代码。

```java
use jni::{objects::JClass, JNIEnv};
 
#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff<'a>(
    env: JNIEnv<'a>,
    _class: JClass,
    code: JString,
) -> JString<'a>
{
    // function body here
}
```

如你所见，JNI 中的返回值仍然作为返回值处理。剩下要做的事就是创建保存结果的 JString。与`get_string()`类似，`JNIEnv`结构还包含一个`new_string()`函数，该函数的功能与名称指代的完全相同。

```rust
// Copy-pasted from earlier snippet 
let code_rust = String::from(env.get_string(code_jni).unwrap());
 
let result = match some_rust_function(code_rust) {
    Ok(value) => format!("OK {}", value),
    Err(e) => format!("ER {:?}", e),
};
return env.new_string(result).unwrap();
```

就像这样，我的 JNI 封装器就完成了。现在，我可以在 Java 代码中调用 Rust 函数，将值传递给调用并接收返回值。

## Rust 的错误处理

尽管代码如预期一样执行，但是我不喜欢出现的`.unwrap()`调用次数。毕竟，错误处理是 Rust 的重要组成部分，仅仅因为我正在进行语言的互操作，并不意味着就可以忽略此事。恰恰相反，我认为两种语言的接触面应该尽可能简单，以防止最终发现一些晦涩的错误是由于互操作性差而引起的。而且，必须检查 Java 的返回值以确定调用是否成功，这使得整个过程使用起来有些笨拙。

我没有重复造轮子，而是对如何更好的将 Rust 的[Result<A, B>](https://doc.rust-lang.org/std/result/enum.Result.html)方式转换成 Java 侧的代码进行了思考。幸运的是，我的 Rust 函数的返回值都是字符串。至于错误，大多数错误要么是不可恢复的，要么是由错误的输入引起的 - 这意味着我可以放弃使用精确的错误代码，而仅仅依靠正确格式的错误信息 - 这又是指字符串。因此`Result<A, B>`可以变成`Result<String, String>`。

## 定义 Java 类

尽管 Java 支持范型(虽然有点[欺骗](https://en.wikipedia.org/wiki/Generics_in_Java#Problems_with_type_erasure)的感觉)，但是我不想从 JNI 中深入了解使用范型的细节。我决定创建一个 Java 类，大致表示`Result<String, String>`语义。

```java
public class Result {
    private boolean ok;
    private String value;
 
    public Result(boolean is_ok, String value) {
        this.ok = is_ok;
        this.value = value;
    }
 
    public boolean isOk() {
        return this.ok;
    }
 
    public boolean isError() {
        return !this.ok;
    }
 
    public String getValue() {
        return this.ok ? this.value : null;
    }
 
    public String getError() {
        return this.ok ? null : this.value;
    }
}
```

尽管完成了这项工作，但与 Rust 相比，它有一些缺点 - 最严重的就是当访问错误的结果变量时返回`null`。由于 null 对于 Java 字符串来说是一个没有问题的值，因此调用`getValue()`可能没有注意并将其传递给其他地方导致在无关紧要的代码中弹出 [NullPointerException](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/lang/NullPointerException.html)。尽管可以通过抛出异常来轻松地改善这个问题，我仍决定使用最好的方式来处理这个问题，以便此处以后永远也不需要修改这部分的代码。

## 从 JNI 中返回一个对象

剩下的唯一一件事就是从 Rust 函数中返回 Result 类的实例。经过一番搜索后，我找到了名为 [NewObject()](https://docs.oracle.com/en/java/javase/11/docs/specs/jni/functions.html#newobject-newobjecta-newobjectv) 的 JNI 函数。该函数有四个参数：

- JNI 环境的句柄
- 我们想要创建的类的句柄
- 构造函数签名
- 构造函数的参数

Rust 函数将 JNI 环境句柄作为其参数之一，因此已经进行了处理。构造函数参数可以作为数组传递，我需要找到另外两个函数参数。

为了获取该函数的句柄，JNI 提供了 [FindClass()](https://docs.oracle.com/en/java/javase/11/docs/specs/jni/functions.html#findclass) 函数。它有两个参数：环境句柄和类的完全限定名 - 简单的说就是类的“导入名”，但是`.`用`/`代替。例如`java.lang.String`变成`java/lang/String`。在本例中是指，`com.startup.hip.Result`变成`com/startup/hip/Result`。

构造函数签名是一个字符串，它很好的描述了构造函数签名需要多少个参数以及哪些类型。乍一看，这让人有些困惑 - 但后来我想起 Java 支持函数重载，并且包括构造函数。由于一个类可能有多个构造函数，所以我必须让 JNI 知道我想使用哪个构造函数。在互联网上搜索了之后，我发现最简单的学习函数签名的方法就是编译 Java 类。然后使用 Java 的反汇编工具：[javap](https://docs.oracle.com/en/java/javase/11/tools/javap.html)。

```console
$ javac android/app/src/main/java/com/startup/hip/Result.java
$ javap -s android/app/src/main/java/com/startup/hip/Result.class
Compiled from "Result.java"
public class com.startup.hip.Result {
  public com.startup.hip.Result(boolean, java.lang.String);
    descriptor: (ZLjava/lang/String;)V

  public boolean isOk();
    descriptor: ()Z

  public boolean isError();
    descriptor: ()Z

  public java.lang.String getValue();
    descriptor: ()Ljava/lang/String;

  public java.lang.String getError();
    descriptor: ()Ljava/lang/String;
}
```

执行了上述的命令，现在我知道了我想要使用的函数签名是`(ZLjava/lang/String;)V`。

在所有步骤都准备就绪之后，是时候创建持有构造函数参数的数组，并调用`NewObject()`。

```rust
fn create_java_result<'e>(
    env: &JNIEnv<'e>,
    is_ok: bool,
    value: &str,
) -> JObject<'e>
{
    let class = env
        .find_class("com/startup/hip/Result")
        .unwrap();
    let args: [JValue<'e>; 2] = [
        JValue::Bool(u8::from(is_ok)),
        JValue::Object(JObject::from(env.new_string(value).unwrap())),
    ];
    env.new_object(class, "(ZLjava/lang/String;)V", &args)
        .unwrap()
}
```

现在，我可以从 native 函数中返回自定义的`Result` Java 类了。

## 使用更通用的解决方案

尽管上面的代码可以很好的实现这个目的，但是它有一个缺点：它显示地采用了布尔值和字符串，要求调用者自己处理 Result 并使用适当的参数调用函数。编写“错误应该尽早返回”的逻辑很繁琐，但是幸运的是，Rust 为此提供了一个解决方案 - [?](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html) 运算符。但是我们的代码从不同的库中调用函数，这些函数又使用了不同的错误类型 - 这意味着我们无法使用`Result<OurType, OurError>`，并且必须执行类似 `Result<OurType, std::error::Error>`的操作 - 这是不可能的，因为 Rust 不允许将特征用作函数的返回类型。

解决此问题的标准方法是使用 [Box<dyn Trait>](https://doc.rust-lang.org/rust-by-example/trait/dyn.html)，但为了使事情变得更加简单，我决定使用 [anyhow](https://crates.io/crates/anyhow) 库，该库允许按我的喜好混合和匹配错误。不管怎样，我可以这样编写代码：

```rust
fn rust_result_to_java_result<'e, T>(
    env: &JNIEnv<'e>,
    result: anyhow::Result<T>,
) -> JObject<'e>
where
    T: Display,
{
    let (is_ok, value) = match result {
        Ok(v) => (true, format!("{}", v)),
        Err(e) => (false, format!("{:?}", e)),
    };
    create_java_result(env, is_ok, value)
}
 
fn actually_do_stuff<'a>(
    env: JNIEnv<'a>,
    code: JString,
) -> anyhow::Result<String>
{
    let code = String::from(env.get_string(code)?);
    let intermediate_value = some_rust_function(code)?;
    other_rust_function(intermediate_value)
}
 
#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff<'a>(
    env: JNIEnv<'a>,
    _class: JClass,
    code: JString,
) -> JObject<'a>
{
    rust_result_to_java_result(actually_do_stuff(env, code))
}
```

更简单了！现在我可以返回任何想要的结果，并将其转换为 Java 类的实例，以供 Java 代码使用。

## 封装

在 Android 中运行 Rust 并不是一件容易的事，但是我对最终找到的解决方案感到满意。我们使用及其普通的 Rust 编写代码并将其编译到共享库中，然后由 JVM 在运行时加载。尽管 JNI 乍一看有点令人生畏，但是使用此标准化解决方案意味着 Java 代码和 Gradle 构建系统都不关心我们的 native 代码是用 Rust 编写的。使用 Cargo 进行交叉编译仍然有些棘手，因为事实证明`cargo-apk`设置了许多[环境变量](https://github.com/rust-windowing/android-ndk-rs/blob/7936944edc699d3e7f380cfa87515f8899ce7027/ndk-build/src/cargo.rs#L6)以使整个过程正常运行。我们的代码还依赖于外部库 - 但是所有的这些都可以通过一堆 shell 脚本来解决。

如果你想要自己尝试一下，我已经准备了一个公共 [Github](https://github.com/suve/rust-on-android/) 仓库，其中包含了一个最小的 Android 应用程序，既包含用 Rust 编写的部分，还依赖于外部的 C 库。该项目的许可证是 [zlib](https://tldrlegal.com/license/zlib-libpng-license-%28zlib%29)。因此可以随意的获取源代码并将其用于你的个人目的。

## 参考

- [Android NDK documentation: other build systems: Autoconf](https://developer.android.com/ndk/guides/other_build_systems#autoconf)
- [crates.io: cargo-apk](https://crates.io/crates/cargo-apk)
- [cargo-apk: ndk-glue/src/lib.rs](https://github.com/rust-windowing/android-ndk-rs/blob/b430a5e274dea8fd7c45e176d5d19c31b73a20ac/ndk-glue/src/lib.rs#L132)
- [cargo-apk: nkd-build/src/cargo.rs](https://github.com/rust-windowing/android-ndk-rs/blob/7936944edc699d3e7f380cfa87515f8899ce7027/ndk-build/src/cargo.rs#L6)
- [Android developer documentation: app manifest: <activity>](https://developer.android.com/guide/topics/manifest/activity-element)
- [Android developer documentation: Activity](https://developer.android.com/reference/android/app/Activity)
- [Android developer documentation: NativeActivity](https://developer.android.com/reference/android/app/NativeActivity)
- [Android developer documentation: Intent](https://developer.android.com/reference/android/content/Intent.html)
- [crates.io: jni](https://crates.io/crates/jni)
- [Java SE 11: JNI specification](https://docs.oracle.com/en/java/javase/11/docs/specs/jni/index.html)
- [Java SE 9: tools: javah](https://docs.oracle.com/javase/9/tools/javah.htm)
- [The Rust Programming Language: Calling Rust Functions from Other Languages](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-rust-functions-from-other-languages)
- [Java SE 11: tools: javap](https://docs.oracle.com/en/java/javase/11/tools/javap.html)
- [Thorn Technologies: Using JNI to call C functions from Android Java](https://www.thorntech.com/2012/08/using-jni-with-java-for-android-sawbix-case-study-part-ii/)
- [Code Ranch: How to create new objects with JNI](https://coderanch.com/t/446953/java/create-NewObject-JNI-methods)
- [Stack Overflow: Java signature for method](https://stackoverflow.com/questions/22038466/jni-signature-for-method)