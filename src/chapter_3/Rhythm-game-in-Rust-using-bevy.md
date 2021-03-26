>* Rhythm game in Rust using Bevy 译文（基于 Rust 用 Bevy 实现节奏大师游戏）
>* 原文链接：https://caballerocoll.com/blog/bevy-rhythm-game/
>* 原文作者：[Guillem Caballero Coll](https://github.com/guimcaballero)
>* 译文来自：[RustMagazine 2021 期刊](https://github.com/RustMagazine/rust_magazine_2021)
>* 译者：[suhanyujie](https://github.com/suhanyujie)
>* ps：水平有限，翻译不当之处，还请指正。
>* 标签：Rust, Bevy,  game, Rhythm game

>2021/2/8 - 77 min read

## 介绍
In this tutorial we'll use the [Bevy](https://bevyengine.org/) game engine to make a rhythm game in Rust. The objective is to show off how to do things in Bevy, specially some more advanced features, like shaders, states, and audio.
>在这个教程中，我们使用 Bevy 引擎用 Rust 实现一个节奏大师游戏。目的是展现如何用 Bevy 实现，特别是一些更高级的功能，如着色器，状态，和音频。

If you want to see the final code before diving in, you can find the repository [here](https://github.com/guimcaballero/bevy_rhythm), and here's a video of how the game works:
>如果你想在进入学习之前看看最终的代码，你可以在这里找到仓库，并且下面是一个操作该游戏视频：

[视频资源](https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4)

This game will be pretty simple: arrows will be flying through the screen, and the player has to press the correct key at the right time to make it disappear. If they do so successfully, they'll gain points. If they don't, the arrow will fall down spinning. Arrows will have different speeds, each of them with a different color. The game will have a menu to select songs, and a small map maker to help create maps for the songs.
>这款游戏很简单：箭头飞过屏幕，玩家必须在正确的时间内按下正确的方向键才能让箭头消失。如果玩家成功地做到了这一点，他们将获得积分。否则，箭头会旋转着掉下来。箭头会有不同地速度，每一个箭头颜色不同。游戏还有一个选择歌曲地菜单，以及一个小地图制作器来帮助创建歌曲地图。

## Bevy
[Bevy](https://bevyengine.org/) is a data-driven game engine built in Rust. It's really straight forward to use, and a joy to work with. It uses [ECS](https://en.wikipedia.org/wiki/Entity_component_system) to manage the games entities and their behaviors.
>[Bevy](https://bevyengine.org/) 是一个基于数据的游戏引擎。它使用起来非常简单，令人愉悦。它使用 [ECS](https://en.wikipedia.org/wiki/Entity_component_system) 来管理游戏实体及其行为。

Bevy has a very welcoming community, so if you have any doubts during this tutorial, check out the [Bevy book](https://bevyengine.org/learn/book/introduction/), look through the [examples](https://github.com/bevyengine/bevy/tree/master/examples), or join the [Official Discord](https://discord.gg/gMUk5Ph) to ask questions!
>Bevy 有一个很受欢迎的社区，所以如果你对本教程有任何疑问，可以查阅 [Bevy book](https://bevyengine.org/learn/book/introduction/)，浏览[示例]](https://github.com/bevyengine/bevy/tree/master/examples)，或者加入[官方的 Discord](https://discord.gg/gMUk5Ph) 进行提问。

If you find any problem in this tutorial, please open an [Issue](https://github.com/guimcaballero/bevy_rhythm/issues) here so I can fix it.
>如果你发现教程中存在错误，请在这里开一个 [Issue](https://github.com/guimcaballero/bevy_rhythm/issues)，我会修正它。

## Prerequisites
> 前期准备

For this tutorial you'll need to be familiar with Rust. You don't need to be an expert, we're not going to use any black magic. Some knowledge of how ECS works is strongly recommended, although you might be able to do without it.
>在本教程中，你需要熟悉 Rust。你不必成为专家，我们不会使用任何的黑魔法。虽然不是必须的，但强烈建议你去了解一下 ECS 的工作原理。

If you want to first checkout some lighter tutorials, I recommend reading [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/) or my [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial/) tutorials, which go over the basics in more detail.
>如果你想阅读一些更轻量的教程，我建议你阅读[基于 Rust，使用 Bevy 实现贪吃蛇](https://mbuffett.com/posts/bevy-snake-tutorial/)，或者[ Bevy 实现国际象棋](https://caballerocoll.com/blog/bevy-chess-tutorial/)教程，可以详细了解基础知识。

Also, we'll be playing around with shaders and [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language) in this tutorial. Knowledge of either is not necessary, as I'll provide the code we're going to use, but knowing GLSL will allow you to change things and make the game yours!
>此外，我们将在本教程中使用着色器和 [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language)。这两种知识不是必须的，因为我会提供要使用的代码，但了解 GLSL 会使你可以修改更多的东西，并让游戏真正属于你自己的。

Here are some recommendations to get started if you've never worked with shaders before:
>如果你之前从未使用过着色器，可以参考下面这些建议开始学习：

* [Shadertoy for absolute beginners](https://www.youtube.com/watch?v=u5HAYVHsasc): Introduction to shaders and using [Shadertoy](https://www.shadertoy.com/).
* [Shadertoy 入门](https://www.youtube.com/watch?v=u5HAYVHsasc)：介绍并使用 [Shadertoy](https://www.shadertoy.com/)。
* [Intro to Shader Coding in Unity - An Improvised Live Course](https://www.youtube.com/watch?v=9WW5-0N1DsI): Introduction to using shaders in Unity. Most of the non-Unity specific knowledge should be applicable here.
* Unity 着色器编码入门 —— [一款即兴的在线课程](https://www.youtube.com/watch?v=9WW5-0N1DsI)：介绍在 Unity 中使用着色器。非 Unity 指定的大部分资料都在这儿。
* [Unity Tutorial: A Practical Intro to Shaders - Part 1](https://www.youtube.com/watch?v=C0uJ4sZelio): Same as above.
* [Unity 教程：着色器的实用介绍 —— 第一部分](https://www.youtube.com/watch?v=C0uJ4sZelio)：与上面类似。

## 创建一个项目
As always, let's start by creating an empty Rust project with `cargo new bevy_rhythm && cd bevy_rhythm`. You can now open `Cargo.toml` with your preferred editor, and add `bevy` as a dependency:
>和往常一样，我们使用 `cargo new bevy_rhythm && cd bevy_rhythm` 创建一个空 Rust 项目。你现在可以打开该 crate 项目。并用你喜欢的编辑器打开 `Cargo.toml`，把 `bevy` 加入到依赖项中：

```rust
[package]
name = "bevy_rhythm"
version = "0.1.0"
authors = ["You <your@emailhere.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bevy = "0.4"
```

## 快速编译
I recommend you enable Fast Compiles, to ensure that the development process doesn't become tedious. Here's what we need for that:
>我建议你启用快速编译，以确保开发过程不会太烦躁。以下是我们需要准备的：

* 1.LLD Linker: The normal linker is a bit slow, so we can swap it out for the LLD Linker to get a speedup:
* LLD 链接器：普通链接器会有点慢，所以我们把其换成 LLD 链接器进行加速：
    * Ubuntu: `sudo apt-get install lld`
    * Arch: `sudo pacman -S lld`
    * Windows: `cargo install -f cargo-binutils and rustup component add llvm-tools-preview`
    * MacOS: `brew install michaeleisel/zld/zld`
* 2.Enable nightly Rust for this project: rustup toolchain install nightly to install nightly, and rustup override set nightly on the project directory to enable it.
* 2.为该项目启用 Rust 的 nightly 版本：rustup 工具链安装 nightly 版，并且 rustup 在项目目录中设置 nightly 进行启用。
* 3.Copy the contents of [this file](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds) into `bevy_rhythm/.cargo/config`.
* 3.把[这个文件](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds)的内容拷贝到 `bevy_rhythm/.cargo/config` 中。

That should be everything, run the game now to compile all of the libraries. At the end, you should see `Hello, world!` in the command line.
>以上就是所有要准备的事情了，现在运行游戏来编译所有的库。编译完成后，你应该在命令行中看到 `Hello, world!`。

Note: If you see that the performance of the game is bad, or you see [assets taking a long time to load](https://github.com/guimcaballero/bevy_rhythm/issues/1), you can run in release mode with `cargo run --release`. Compile times might be a bit longer, but the game will run much smoother!
>注意：如果你看到游戏性能很差，或者看到[加载资源很慢](https://github.com/guimcaballero/bevy_rhythm/issues/1)，你可以用 `cargo run --release` 的编译模式下运行。编译时间可能会稍长一些，但游戏运行会更加流畅！ 

## 开始
The first step for any Bevy game is to add the small boilerplate code to start the app. Open `main.rs`, and replace the existing `main` function with the following:
>任何 Bevy 游戏的第一步都是增加小段样板代码来启动应用的。打开 `main.rs`，并将现有的 `main` 函数替换为下面的内容：

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .run();
}
```

If you run the program using `cargo run`, you'll see an empty window open up:
>如果你使用 `cargo run` 运行程序，你会看到一个空窗口：

![](https://caballerocoll.com/images/bevy_empty_window.png)

This step sets up a Bevy `App`, adding the default plugins. This will include things like transforms, input, windows and everything else we need to get a game going. In case you don't need some of these features, Bevy is modular enough to allow you to select which ones you want to enable. We'll be using all of them, so we use `add_plugins` with `DefaultPlugins`.
>这一步设置一个 Bevy `App`，添加默认插件。这将包括转换，输入，窗口等游戏运行所需的元素。如果你不需要这些功能， Bevy 是模块化的，你可以选择只开启你需要的功能。我们要新增这些插件，所以需要使用 `add_plugins` 和 `DefaultPlugins`。

We've also added the two resources: `Msaa` and `WindowDescriptor`, to configure anti-aliasing, and the window size and title respectively. Last thing we added is the `exit_on_esc_system` that comes with Bevy, which will take care of closing the game when we press the escape key.
>我们还添加了连个资源：`Msaa` 和 `WindowDescriptor`，分别用于配置 anti-aliasing，以及窗口大小和标题。最后，我们添加了 Bevy 的 `exit_on_esc_system`，它的作用是按下 esc 键时关闭游戏。

## Bevy 中的 ECS
Here's a small introduction to how ECS works in Bevy. Feel free to [skip this chapter](https://caballerocoll.com/blog/bevy-rhythm-game/#adding-a-setup-system) if you already know how it works. This is completely unrelated to our game, I'll be using examples from the [Bevy book](https://bevyengine.org/learn/book/getting-started/ecs/) to illustrate how it works. You don't have to copy the code here, just read through it and make sure you understand what's going on.
>下面是 ECS 如何在 Bevy 中工作的介绍。如果你已经知道它是如何工作的，可以[跳过本节](https://caballerocoll.com/blog/bevy-rhythm-game/#adding-a-setup-system)。这和我们的游戏无关，我将使用 [Bevy book](https://bevyengine.org/learn/book/getting-started/ecs/) 中的例子来说明它是如何运作的。你不需要复制这里的代码，只需读懂它即可。

Bevy's ECS is a fork of [hecs](https://github.com/Ralith/hecs). It uses normal Rust structs as components, without the need to add macros or any complicated stuff. For example, we could have:
>Bevy 的 ECS 是 [hecs](https://github.com/Ralith/hecs) 的一个分支版本。它使用 Rust 结构体作为组件，不需要添加宏或其他复杂的东西。例如：

```rust
// 有两个字段的结构体组件
struct Position { 
    x: f32,
    y: f32
}

// 元组组件
struct Name(String);

// We can even have marker components
// 我们甚至可以使用标记组件
struct Person;
```

Systems are just normal Rust functions, that have access to `Querys`:
>这个系统中可以使用正常的 Rust 函数，访问 `Querys`：

```rust
fn set_names(mut query: Query<(&Position, &mut Name), With<Person>>) {
    for (pos, mut name) in query.iter_mut() {
        name.0 = format!("position: ({}, {})", pos.x, pos.y);
    }
}
```

A query allows us to access the entities that have the provided components. In the previous example, `query` allows us to iterate over the `Position` and a `Name` components of entities that have those components and also a `Person` component. As we're using `&mut Name` instead of `&Name`, we can modify it. If we tried to modify it while using the latter, Rust would complain.
>一次查询可以让我们访问组件中所有实体。在前面的示例中，`query` 参数允许我们迭代包括 `Person` 组件在内以及`Position` 和 `Name` 等组件实体。因为我们用 `&mut Name` 替代 `&Name`，所以可以对实体修改。如果在修改时使用该值，Rust 则会报错。

Sometimes we might want to have a system that only runs once at the beginning of the game. We can do that with startup systems. Startup systems are declared exactly the same as normal systems, the only difference is how we add them to the game, which will be shown later. Here's an example of a startup system that uses `Commands` to spawn some entities:
>有时候我们想要只在游戏开始时运行一次的机制。我们可以通过“启动系统”来做到这一点。“启动系统”和“普通系统”完全一样，唯一的区别是我们将如何把它加到游戏中，这会在后面进行详细讲解。下面是一个使用 `Commands` 生成一些实体的“启动系统”：

```rust
fn setup(commands: &mut Commands) {
    commands
        .spawn((Position { x: 1., y: 2. }, Name("Entity 1".to_string())))
        .spawn((Position { x: 3., y: 9. }, Name("Entity 2".to_string())));
}
```

Bevy also has Resources, which allow us to keep global data. For example, the built-in `Time` resource provides us with the current time in the game. To use resources in a system, we use `Res`:
>Bevy 也有资源的概念，它可以保存全局数据。例如，内置的 `Time` 资源给我们提供游戏中的当前时间。为了在系统中使用这类资源，我们需要使用 `Res`：

```rust
fn change_position(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut pos in query.iter_mut() {
        pos.x = time.seconds_since_startup() as f32;
    }
}
```

Making our own resource is easy as well:
>我们自定义自己的资源也很简单：

```rust
// 一个简单的资源
struct Scoreboard {
    score: usize,
}

// Another resource, this one implements Default
// 又一个资源，它实现了 Default trait
#[derive(Default)]
struct OtherScore(f32);
```

We have two options to initialize resources: the first on is to use `.add_resource` and to provide the struct we want, the other option is to use `.init_resource`, if it implements either the `Default` or the `FromResources` trait.
>我们有两种方法初始化资源：第一种是使用 `.add_resource` 并提供我们需要的结构体，另一种是在实现 `Default` 和 `FromResources` 时使用 `.init_resource`。

And here's how we'd add these systems to our game:
>下面我们把它们加到游戏中：

```rust
fn main() {
    App::build()
        // 新增资源的第一种方法
        .add_resource(Scoreboard { score: 7 })
        // Second way of adding a resource, will be initialized using Default
        // 第二种方法，通过 Default 的初始化加载资源
        .init_resource::<OtherScore>()

        // Add a startup system, only runs once at the start
        // 增加“启动系统”，游戏启动时只会运行一次
        .add_startup_system(setup.system())
        // Add a normal system, runs once every frame
        // 增加一个“普通系统”，每一帧都会运行一次
        .add_system(set_names.system())
        .add_system(change_position.system())
        .run();
}
```

Another cool thing Bevy has are Plugins, which we've already seen when we used `DefaultPlugins` in the previous section. Plugins allow us to wrap features that belong together, which then let's us enable and disable them together easily. Plugins also provide organization, which is the main purpose we'll be creating our own in this tutorial.
>bevy 还有一个很酷的东西是插件，我们在上一节使用 `DefaultPlugins` 时看到了。插件可以让我们包装一些特性，这可以让我们很容易地启用和禁用它，插件也提供了组织功能，这也是我们在这篇教程中自定义插件地主要功能点。

If there's some stuff that isn't clear right now, don't worry too much, we'll go over all of this in more detail later.
>如果有些东西不清楚，不用担心，我们会在后面更详细地解释所有内容。

## Adding a setup system
>增加系统设置

Every game needs a camera to render the objects, so we'll begin by adding a startup system that spawns a camera. Since this is a 2D game, we'll use the aptly named `Camera2dBundle`.
>每个游戏都需要一个相机来渲染对象，所以我们将从如何添加一个生成相机的“启动系统”开始。因为这是一款 2D 游戏，所以我们要使用 `Camera2dBundle`。

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        // 设定[抗锯齿](https://cn.bing.com/search?q=%E7%BB%98%E5%88%B6+%E6%8A%97%E9%94%AF%E9%BD%BF&qs=n&form=QBRE&sp=-1&pq=%E7%BB%98%E5%88%B6+%E6%8A%97%E9%94%AF%E9%BD%BF)，samples 参数值为 4
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        // 设定 WindowDescriptor 资源，定义我们需要的标题和窗口大小
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system()) // <--- New
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

A bundle is a collection of components. In this case, `Camera2dBundle` will create an entity with the `Camera`, `OrthographicProjection`, `VisibleEntities`, `Transform` and `GlobalTransform`. Most of these are internal components that we don't really need to play with, so we can just use the abstracted `Camera2dBundle` to add them all for us.
>bundle 是组件的集合。在本例中，`Camera2dBundle` 将创建一个包含 `Camera`，`OrthographicProjection`，`VisibleEntities`，`Transform` 和 `GlobalTransform` 的 实体。其中大部分是我们玩游戏时不需要用到的，所以我们使用抽象的 `Camera2dBundle` 添加组件。

Note: We could add all of the components individually instead of using the bundle, by using a tuple like so:
>注意：我们还可以使用一个元组来单独添加所有组件，而非 bundle：

```rust
fn setup(commands: &mut Commands) {
    commands.spawn((Camera::default(), OrthographicProjection::default(), VisibleEntities::default(), Transform::default(), GlobalTransform::default()));
}
```

This code doesn't actually work, as we would need to set up some of the fields in the camera and projection components, but I think it illustrates how using a bundle is similar to adding all of the structs by themselves in a tuple.
>这段代码实际上还不能运行，因为我们还需要在 camera 和 投影组件中设置一些字段，但我认为它明确地体现了使用 bundle 和元组来添加结构是很相似的。

## 加载精灵
In this first section we'll be adding some sprites and making them move around. For that, we need to create an `assets` directory, where we'll store the [images](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/images) and [fonts](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/fonts). We'll have two subfolders inside, images and fonts. You can go to the previous links to download the files from the GitHub repository.
>在这部分中，我们会添加一些“精灵”，让它们四处移动。为此，我们需要创建一个 `assets` 目录，我们将存储一些[图像](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/images)和[字体文件](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/fonts)。目录中有两个子文件夹，图像和字体。你可以点击前面提到的链接，从 GitHub 仓库下载。

Your assets folder should look like this:
>你的资源目录应该如下所示：

```
assets
├── fonts
│   └── FiraSans-Bold.ttf
└── images
    ├── arrow_blue.png
    ├── arrow_border.png
    ├── arrow_green.png
    └── arrow_red.png
```

We'll use the colored arrows to indicate the different arrow speeds, and we'll use the bordered one to mark the targets.
>我们将使用带颜色的箭头来表示不同速度的箭头，并使用带边框的箭头来标记目标。

With our assets taken care of, we can start coding some behavior. We'll make a new file called `arrows.rs` that will keep all the systems that relate to spawning, moving and despawning arrows. The first thing will be a resource that keeps the materials for the arrow sprites, this way we don't have to load them every time we want to create an arrow:
>有了这些静态资源，我们旧可以开始编写一些行为了。我们将创建一个 `arrows.rs` 文件，它将包含生成，移动，清理箭头等相关操作。首先要做的是为“箭头精灵”保留资源，这样我们就不必在每次创建箭头时加载它们：

```rust
use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
/// 为箭头保留材料和资源
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}
impl FromResources for ArrowMaterialResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}
```

By implementing the `FromResources` trait, Bevy will take care of initializing the resource correctly once we do `.init_resource::<ArrowMaterialResource>()`, loading the images in the process.
>通过实现 `FromResources` trait，在我们执行 `.init_resource::<ArrowMaterialResource>()` 时，Bevy 会管理并初始化资源，在进程中加载图片。

As you can see, instead of actually holding the `ColorMaterials`, the resource has `Handle<ColorMaterial>`. This way, when we create the arrows, we can give them the same handle, and they'll all share the same materials, instead of each having their own.
>如果所看到地，实际的资源加载是 `Handle<ColorMaterial>` 而不是 `ColorMaterials`。这样，当我们创建箭头实例时，我们可以使用对应的 handle，并且它们将复用已存在的资源，而不是每个都各自独有一份。

## Spawning and moving arrows
>生成并移动箭头

The next thing we'll be working on is spawning the arrow sprites and moving them across the screen. We'll start by making a system that spawns an arrow once a second. The arrow will have an empty component called `Arrow`:
>我们接下来要做的是生成箭头并在屏幕上移动它们。我们从每秒生成一个箭头的系统开始。箭头会包含一个名为 `Arrow` 的空（结构体）组件：

```rust
/// Arrow component
/// 箭头组件
struct Arrow;

/// Keeps track of when to Spawn a new arrow
struct SpawnTimer(Timer);

/// Spawns arrows
/// 生成箭头
fn spawn_arrows(
    commands: &mut Commands,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(-400., 0., 1.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(140., 140.)),
            transform,
            ..Default::default()
        })
        .with(Arrow);
}
```

In this system we're making use of a `Timer`, which is the best way to do repeated actions every `x` seconds in Bevy. We're using the [newtype pattern](https://rust-unofficial.github.io/patterns/patterns/newtype.html) to provide encapsulation, allowing us to distinguish our `SpawnTimer` from other possible timers. We have to initialize it using something like `.add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))`, which we'll do a bit later. Passing `true` as a parameter makes it so that the timer repeats after it's finished.
>在这个系统中，我们使用了 `Timer`，这是 Bevy 中执行每隔 `x` 秒重复操作的最佳方式。我们使用 [newtype 模式](https://rust-unofficial.github.io/patterns/patterns/newtype.html)进行封装，这样我们能够把 `SpawnTimer` 与其他的定时器区分开。我们需要使用形如 `.add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))` 的方式进行初始化，稍后会进行。将 `true` 作为参数值传递表示计时器结束时会再次重复执行。

To use a timer, we have to manually call it's `tick` method with the time passed since the last time we called it, and we can then use `just_finished` to see if the timer is done. Effectively, what we're doing is ensure that the `spawn_arrows` system only runs once a second, by having an early return that checks if the timer is done.
>要使用计时器，我们必须手动调用它的 `tick` 方法，入参 time 是距离上次调用所间隔的时间差，然后我们可以使用 `just_finished` 来查看定时器是否完成。实际上我们所做的是提前检查定时器是否完成来确保 `spawn_arrows` 系统每秒只运行一次。

The rest of the system creates a `Transform` component, that we'll add to the arrow, and it spawns a `SpriteBundle` to spawn the arrows, giving it the red texture from the `ArrowMaterialResource`. To the arrow we're adding the `Arrow` component using the `with` method in `Commands`. This way, the entity we're creating will have all of the `SpriteBundle` components plus our `Arrow` component.
>系统的其余部分将创建一个 `Transform` 组件，我们将其添加到箭头组件中，它会返回 `SpriteBundle` 来生成箭头，并给箭头实体一个来自 `ArrowMaterialResource` 的红色纹理。我们使用 `Commands` 中的 `with` 方法添加了 `Arrow` 组件。这样，我们创建的实体将拥有所有的 `SpriteBundle` 和 `Arrow` 组件。

Note: this system is just temporary, and will be replaced by something that spawns the arrows at certain specified times.
>注意：这个系统只是临时的，并且它会被在某个特定时间内生成的东西所覆盖。

Now, those arrows we're spawning are just standing there, so let's make them move to the right with another system:
>现在，我们生成的那些箭头就在那了，我们需要用另一个系统让它们向右移动：

```rust
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}
```

`move_arrows` uses a `Query` to take all of the entities with a `Transform` and an `Arrow` component, and changes their translation to be a bit to the right, by increasing the x coordinate. We're also using `Time::delta_seconds()` to increase the distance according to how much time has passed since the last frame.
>`move_arrows` 使用 `Query` 来获取所有带有 `Transform` 和 `Arrow` 组件的实体，并通过增加 x 坐标值来将它们向右移动一点点。我们还使用了 `Time::delta_seconds()` 来根据当前帧到上一帧的时间来增加距离。 

We'll join all of these systems, `ArrowMaterialResource`, and `SpawnTimer`, into a Plugin:
>我们把这些 `ArrowMaterialResource` 和 `SpawnTimer` 等系统连接到一个插件中：

```rust
pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Initialize Resources
            .init_resource::<ArrowMaterialResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            // Add systems
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}
```

We can finally change `main.rs` to contain the following:
>我们现在可以将 `main.rs` 改为如下内容：

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin) // <--- New
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

The only thing we have changed is adding the `.add_plugin(ArrowsPlugin)`, so that all of the systems and resources in `arrows.rs` are added correctly.
>我们需要做的只是增加 `.add_plugin(ArrowsPlugin)`，这样所有的系统和资源就被正确地集成在 `arrows.rs` 中。

If you run the game now, you should see arrows flying across the screen:
>如果你运行程序，你会看到箭头在屏幕上飞舞：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_red_arrows_moving.mp4)

## Types and constants
>类型和常量

There's some values that we have hardcoded in the previous section. As we're going to be reusing them a bit, we're going to make a small module where we'll keep our constants. Make a new file called `consts.rs`, and add the following to it:
>我们在上一节中对一些值硬编码了。因此我们需要重新使用它们，我们要新建一个小模块来保存我们的常数。创建一个名为 `consts.rs` 的文件，并添加以下内容：

```rust
/// Speed at which a Slow arrow moves
/// 箭头移动的速度
pub const BASE_SPEED: f32 = 200.;

/// X coordinate value at which arrows spawn, should be out of screen
/// 箭头生成时的 X 坐标值，应该在屏幕之外
pub const SPAWN_POSITION: f32 = -400.;

/// X coordinate value where the arrows should be clicked
/// 箭头应该被正确点击时的 X 坐标值
pub const TARGET_POSITION: f32 = 200.;

/// Margin of error for clicking an arrow
/// 点击箭头时的容错间隔
pub const THRESHOLD: f32 = 20.;

/// Total distance traveled by an arrow, from spawn to target
/// 箭头从刷出到目标的总距离
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;
```

Some of these constants won't be used until a bit later. Add `mod consts` in `main.rs`, to import the module and make it available. We can replace those values in `spawn_arrows` and `move_arrows` in `arrows.rs` to use the constant, like so:
>其中一些常数稍后才会用到。在 `main.rs` 中增加 `mod consts`，以导入模块使其可用。我们可以在 `arrows.rs` 中的 `spawn_arrows` 和 `move_arrows` 替换掉对应硬编码的值。

```rust
use crate::consts::*;

fn spawn_arrows(
    commands: &mut Commands,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 0., 1.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(140., 140.)),
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

/// Moves the arrows forward
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * BASE_SPEED;
    }
}
```

We now have arrows moving across the screen, but at the moment they're all facing the same way, going at the same speed and all have the same color. To be able to tell them apart, we'll create two different enums, one for Directions (Up, Down, Left, Right) and one for Speed (Slow, Medium, Fast).
>现在我们的箭头在屏幕上移动，但他们都面向相同的方向、相同的速度移动，且颜色相同。为了能够区分它们，我们将创建两个不同的枚举，一个用于表示方向（上、下、左、右），一个表示速度（慢、中、快）。

Note: We're calling it `Directions` instead of `Direction`, because the latter is a [Bevy enum](https://docs.rs/bevy/0.4.0/bevy/prelude/enum.Direction.html). By calling it a slightly more awkward name, we're saving ourselves the trouble of having to tell them apart.
>注意：我们把它叫做 `Directions` 而非 `Direction`，因为后者是一个[ Bevy 枚举](https://docs.rs/bevy/0.4.0/bevy/prelude/enum.Direction.html)。通过给它取一个稍微不一样的名字，我们可以省去如何区分它们的麻烦。

Let's create a new file called `types.rs`, where we'll put these enums:
>让我们创建一个 `types.rs` 文件，并把上面提到的枚举值放于其中：

```rust
use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
impl Directions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// Returns the correct y coordinate for an arrow with this direction
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Left => -50.,
            Directions::Right => -150.,
        }
    }
}
```

First, we've added the `Directions` enum. We've implemented three different methods to it.
>首先，我们添加 `Directions` 枚举。我们已经实现了三种不同的方法。

`key_just_pressed`, which checks if a key associated to the direction is being pressed. I've decided to add `D, F, J, K` as possible keys too, as the arrow keys on my keyboard are a bit small. Feel free to replace these with whatever else you want, like `W, S, A, D` if you're more of an FPS person, or `K, J, H, L` if you're living the VIM life.
>`key_just_pressed`，用于检查被按下的方向键。我已经决定增加 `D, F, J, K` 这些可能的键，因为我键盘上的方向键比较小。如果你是 FPS 玩家，你可以使用 `W, S, A, D`，或者 VIM 世界的 `K, J, H, L` 来替代它们。

Note: If you're not very comfortable with iterators, here's how `key_just_pressed` would look like using a more traditional approach:
>注意：如果你不太习惯使用迭代器，下面是用传统的方法实现 `key_just_pressed` 的样子：

```rust
/// Checks if a key that corresponds to this direction has been pressed
/// 检查与方向对应的按键是否被按下
pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
    match self {
        Up => input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::D),
        Down => input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::F),
        Left => input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::J),
        Right => input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::K),
    }
}
```

`rotation` returns how much we have to rotate an arrow sprite to get it pointing in the correct direction. `y` returns the `y` coordinate for the arrow. I've decided to have the arrows be in the order `Up, Down, Left, Right`, but you can change them here if you prefer some other order.
>`rotation` 表示我们需要将“箭头精灵”旋转多少度以将其指向正确的方向。`y` 表示箭头的 `y` 坐标值。我决定把箭头的顺序调整为 `Up, Down, Left, Right`，但如果你喜欢其他顺序，你可以自己修改。

```rust
#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// Returns actual speed at which the arrow should move
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}
```

Next, we've added the `Speed` enum. We've implemented two methods: a multiplier method, which returns how much more than `BASE_SPEED` an arrow should move, and `value`, which performs the multiplication.
>接下来，我们添加了 `Speed` 枚举。我们实现了两个方法：一个是乘法，它表示箭头应该相对于 `BASE_SPEED` 所移动的距离；另一个是 `value`，它是执行乘法运算得到的值。

That was a bit of code, but nothing too complicated I hope! The next types we're going to add are `ArrowTime` and `SongConfig`. The first one will keep track of when an arrow needs to be spawned, and with which direction and speed. The second will keep the list of all the arrows:
>这是部分代码，我不希望特别复杂！接下来要添加的类型是 `ArrowTime` 和 `SongConfig`。前面一个记录何时生成一个箭头，以及它的方向和速度。第二个将保存所有箭头实体的列表：

```rust
#[derive(Clone, Copy, Debug)]
/// Keeps track of when each arrow should spawn and it's speed and direction
/// 跟踪记录箭头应该在什么时候生成，以及箭头的速度和方向。
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

#[derive(Debug)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}
```

There's an issue with our `ArrowTime`. Internally, we need to know at which time to spawn the arrow, but to make it we would like to specify at which time it has to be clicked. Since each arrow has a different speed, it's not enough to just subtract some seconds. To deal with this issue, we'll make a `new` function, that takes the `click_time`, `speed`, and `direction`, and sets the corresponding `spawn_time`:
>我们的 `ArrowTime` 有个问题。在内部，我们需要知道箭头什么时候生成，但在生成它时，我们希望指定应该在什么时候点击它。因为每个箭头都有不同的速度，所以仅仅减去几秒是不够的。为了解决这个问题，我们要创建一个 `new` 函数，包含 `click_time`，`speed` 和 `direction`，并设置相应的 `spawn_time`：

```rust
impl ArrowTime {
    fn new(click_time: f64, speed: Speed, direction: Directions) -> Self {
        let speed_value = speed.value();
        Self {
            spawn_time: click_time - (DISTANCE / speed_value) as f64,
            speed,
            direction,
        }
    }
}
```

To test things we'll also make a function that returns a hard-coded `SongConfig` with some arrows with different speeds and directions:
>为了进行测试，我们将创建一个函数，它返回硬编码的 `SongConfig`，其中包含了不同的速度和方向的箭头：

```rust
pub fn load_config() -> SongConfig {
    SongConfig {
        arrows: vec![
            ArrowTime::new(1., Speed::Slow, Directions::Up),
            ArrowTime::new(2., Speed::Slow, Directions::Down),
            ArrowTime::new(3., Speed::Slow, Directions::Left),
            ArrowTime::new(4., Speed::Medium, Directions::Up),
            ArrowTime::new(5., Speed::Fast, Directions::Right),
        ],
    }
}
```

Finally, we can go into `main.rs` and change the `setup` system into the following:
>最后，我们可以进入 `main.rs` 并将 `setup` 系统改成下方所示：

```rust
mod types;

fn setup(commands: &mut Commands) {
    let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(config);
}
```

Note: We use `insert_resource` instead of `add_resource` or `init_resource`, as these last two are for `AppBuilder`, while the first one is used in `Commands`.
>注意：我们使用 `insert_resource` 替代 `add_resource` 或 `init_resource`，因为后者是 `AppBuilder`，前者是用在 `Commands` 中。

If we run the game now, nothing has changed, but it still works, which is always great! Let's go into `arrows.rs` and change it so that it spawns arrows according to the list in `SongConfig`.
>如果我们现在运行游戏，没有任何变化，但仍然是能运行的，这很棒！我们进入 `arrows.rs` 文件，修改它使它能根据 `SongConfig` 中的列表生成箭头。

## Spawning arrows on time
>定时生成箭头

Now that we have a list of arrows to spawn, we can remove all of our timer stuff and change the `spawn_arrows` system to check what arrows it should spawn each frame.
>现在我们有了一个要生成的箭头列表，我们可以删除所有定时器的内容，并修改 `spawn_arrows` 系统来检查每一帧刷出的箭头。

A first implementation we could come up with would loop through all of the arrows in `SongConfig`, and check which ones should be spawned in the current frame. This would work, but we would be looping over a possibly large list each frame. It's not much of an issue when we only have the 5 arrows we have hardcoded, but a song could be more than a 1000 arrows long, and even though computers are fast, players certainly won't appreciate us needlessly heating up their CPUs.
>我们可以想到的第一个实现是循环遍历 `SongConfig` 中的所有箭头，并检查哪些箭头应该在当前帧中生成。这是可行的，但我们会在每一帧都循环遍历一个可能会很大的数组。我们硬编码的只有 5 个箭头，这不成问题，但一整首歌的情况下，箭头可能会超过 1000 个，就算电脑很快，玩家也不希望游戏让它们的 CPU “热”起来。

Instead, we're going to assume that the arrows in `SongConfig` are sorted. We'll need to take care of actually sorting them before starting the song, but that's easy enough. Knowing that, we can check only the first arrow on the list, and if it should be spawned, we also check the next one, repeating until we reach an arrow that doesn't need to be spawned on that frame. Since the arrows are ordered, if an arrow doesn't need to be spawned, neither of the following arrows will either. After that, we'll need to remove the arrows we have spawned from the beginning of the list.
>相反，我们将假设 `SongConfig` 中的箭头是有序的。我们需要在歌曲开始前将它们进行排序，这很简单。了解了这一点，我们只能先检查列表中的第一个箭头，如果它应该被生成出来，我们也会检查下一个箭头，一次类推，直到我们到达那个不需要再生成的箭头为止。由于箭头是有序的，如果一个一个箭头不需要生成，那么其后的箭头也无需生成。在这之后，我们需要移除从列表开始的所有被生成的箭头。 

We're also going to add `Speed` and `Directions` as fields for `Arrow`:
>我们还需要给 `Arrow` 新增 `Speed` 和 `Directions` 字段：

```rust
// At the top
use crate::types::*;

/// “精灵实体”上的组件
struct Arrow {
    speed: Speed,
    direction: Directions,
}

/// 生成箭头
fn spawn_arrows(
    commands: &mut Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window
    // 我们得到了从启动到当前的时间（secs）以及到最后一次迭代的时间（secs_last），这样我们就可以检查是否有箭头应该在这个窗口中生成。

    // Song starts 3 seconds after start, so we subtract 3 seconds
    // 歌曲在启动后 3 秒开始，所以减去 3 秒。
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    // 需要从列表中产生和删除箭头的计数器
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            // 根据速度得到与之匹配的箭头素材（纹理）
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow acording to direction
            // 按一定的方向旋转箭头
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands
                .spawn(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(140., 140.)),
                    transform,
                    ..Default::default()
                })
                .with(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
    // 移除从列表中生成出来的箭头
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}
```

That's a bit of code, so let's go through it.
>上面这段代码，我们来分析一下它。

At the beginning of the system we first get how many seconds have passed since the start of the game, and the time when this system last run. We do this using [delta_seconds_f64](https://docs.rs/bevy/0.4.0/bevy/core/struct.Time.html#method.delta_seconds_f64), which returns the time that has passed since the last game update. With this two values, we can know which arrows we should spawn. As Bevy doesn't update our game every nanosecond (not that any game engine does that), simply checking if `spawn_time` is equal to the current time would cause us to skip arrows. For example, we might have an arrow with a spawn time set to `3.0`. Bevy might run this system once at time `2.99` and then at `3.01`. Since our arrow is set to spawn at `3.0`, it wouldn't match any of the times we run the system, and so it would never get spawned.
>在“系统”开始时，我们先获取游戏已经开始多久了，以及“系统”最后一次运行的时间点。我们使用 [delta_seconds_f64](https://docs.rs/bevy/0.4.0/bevy/core/struct.Time.html#method.delta_seconds_f64) 来获取，它返回自最后一次游戏更新以来的时间。有了这两个值，我们就能知道该生成哪个箭头。因为 Bevy 不会每纳秒都更新（不代表所有的游戏引擎），所以如果只是简单地检查 `spawn_time` 是否等于当前时间会导致我们跳过需要处理地箭头。例如，我们可能有一个箭头，它刷出的时间被设为 `3.0`。Bevy 可以在 `2.99` 时运行这个“系统”，然后 `3.01` 时运行一次。由于箭头被指定为在 `3.0` 时生成，它就与运行“系统”的时间不匹配，导致它永远不会生成。

With our method here, at the beginning of the system we check for the current time, and the last time, so for our example, the second time we run, we would have `secs = 3.01` and `secs_last = 2.99`, and since our arrow's spawn time is over `secs_last` but under `secs`, we do spawn it. Nice!
>我们换个方法，在“系统”开始时检查当前时间和最后结束时的时间，对于上面的举例，在第二次运行该“系统”时，就会有 `secs = 3.01` 以及 `secs_last = 2.99`，因为我们的箭头产生的时间超过 `secs_last`，但小于下一次的 `secs`，所以能够生成。大功告成！

With that, we can make a small change in `move_arrows` so that it takes speed into account, using the `Speed::value()` method we created before:
>有了这个，我们可以对 `move_arrows` 做一下小修改，让它兼顾速度的影响，可以使用我们之前创建的 `Speed::value()` 方法：

```rust
/// Moves the arrows forward
/// 把箭头向前移动
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();
    }
}
```

Cool, we now have each arrow displayed with it's correct color and moving at the speed it should:
>很酷，现在每个箭头都显示了正确的颜色，并以相应的速度移动：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_colored_arrows_moving_at_speed.mp4)

## Adding target arrows
>增加目标区域箭头

We're now going to use our `border_texture` to create the target arrows, so the players can know when they should press the button. For that, we'll make another startup system, `setup_target_arrows`, and a marker component, `TargetArrow`:
>现在我们将使用 `border_texture` 去创造目标箭头，以便玩家能够知道何时应该按下按键。为此，我们将创建另一个“启动系统”，`setup_target_arrows` 以及一个标记组件，`TargetArrow`：

```rust
struct TargetArrow;

fn setup_target_arrows(commands: &mut Commands, materials: Res<ArrowMaterialResource>) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                material: materials.border_texture.clone(),
                sprite: Sprite::new(Vec2::new(140., 140.)),
                transform,
                ..Default::default()
            })
            .with(TargetArrow);
    }
}
```

To create the four arrows, we make an array with the four directions, then loop through it to spawn arrows with the `border_texture` and the empty `TargetArrow` component.
>为了创建四个箭头，我们创建了一个有四个方向值的数组，然后循环调用 `border_texture` 和空的 `TargetArrow` 组件。

Don't forget to add `setup_target_arrows` as a startup system in `ArrowsPlugin`:
>不要忘记在 `ArrowsPlugin` 中添加 `setup_target_arrows` 作为“启动系统”：


```rust
pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .add_startup_system(setup_target_arrows.system())
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}
```

And there we go! We now have our target arrows ready.
>好了，我们现在把目标“箭头”准备好了。

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_target_arrows.mp4)

## Despawning arrows when pressed
>按下时移除箭头

Now that we have target arrows, let's implement a system that will despawn the arrows if the correct key is clicked while the arrow is inside the threshold. We'll make a new system, called `despawn_arrows`:
>现在我们有了目标箭头，我们接下来要实现一个“系统”，它的作用是，当箭头刷出时，并且如果在特定的阈值内，用户点击了正确的操作键，箭头就会消失。我们将创建一个名为 `despawn_arrows` 的“新系统”：

```rust
/// Despawns arrows when they reach the end if the correct button is clicked
/// 用户在箭头到达尽头前按下正确的按键，箭头消失。
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        // 检查按下按键时是在箭头的特定阈值内
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);
        }

        // Despawn arrows after they leave the screen
        // 当箭头离开箭头时，箭头消失
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
        }
    }
}
```

We use a `Query` to get all of the entities with a `Transform` and an `Arrow` component. We've also added `Entity` to the query, which gives us access to the entity's "id", which we can then use in `Commands::despawn()` to despawn the entity. We then loop through the arrows, and check if the x coordinate is inside the threshold for clicking, and if so, despawns the arrow. It also has a second check, to despawn an arrow after it has been missed and it has left the screen. It's done in a bit of a lazy way, with `2. * TARGET_POSITION`.
>我们使用 `Query` 来查询所有实现了 `Transform` 和 `Arrow` 的实体。我们在查询中添加了 `Entity`，这样可以访问实体的“id”，然后我们可以在 `Commands::despawn()` 中使用它来消除实体。然后我们循环所有箭头，并检查 x 坐标是否在点击的阈值内，如果是，则消除箭头。还有第二个检查，当箭头被错过离开屏幕时，它在最后也会被消除。它是在 x 坐标值大于等于 `2. * TARGET_POSITION` 时消除。 

Remember to add this system to `ArrowsPlugin` with `.add_system(despawn_arrows.system())`, and with that done, you can run the game and actually kind of play something that when we squint might resemble a game!
>记得用 `.add_system(despawn_arrows.system())` 将“系统”添加到 `ArrowsPlugin` 中，这样，当我们有点乏的时候，可以运行它并玩一玩这个不太完整的游戏！

## Adding basic UI
>增加基础 UI

In this section we'll implement some basic UI, which for now will just show the current time in the song. We'll keep all of it in `ui.rs`:
>在这一节中，我们将实现一些基本的 UI，目前只是显示了歌曲中的当前时间。我们会把它保存在 `ui.rs` 中：

```rust
use bevy::prelude::*;

fn setup_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "Time: 0.0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TimeText);
        });
}

struct TimeText;
```

In this system we've used parenting, which makes a child entity have it's transform be relative to it's parent entity. We add children to a parent entity using the aptly named `with_children`, which takes a closure that will receive a `ChildBuilder`, a struct that is very similar to `Commands`, and allows us to spawn entities which will automatically be set as children. In this case, we're creating a `NodeBundle` as a parent entity, and we're adding a `TextBundle` as a child to it. We're using the CSS-like `Style` component to make the parent node sit at the top left of the screen. To the text entity we're adding a `TimeText` marker component, so that we can query it and change it each frame.
>在这个系统中，我们使用了父子模式（parenting），它产生一个子实体，子实体可以相对于父实体进行转换。当我们把子实体加到父实体中后，给它一个合适的命名 `with_children`，它的参数是一个闭包，闭包接受一个类似于 `Commands` 的结构体类型 `ChildBuilder` 参数。在这个例子中，我创建了一个 `NodeBundle` 作为父实体，并添加了 `TextBundle` 作为子实体。我们使用类似于 css 风格的 `Style` 组件让父节点坐落在屏幕的左上角。我们给文本实体增加了 `TimeText` 标记组件，这样我们就可以查询它，并且可以在任意帧中修改它。

We can now add a system which updates the text each frame:
>现在，我们可以添加一个“系统”，它可以在每一帧中更新文本：

```rust
fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.value = format!("Time: {:.2}", secs);
    }
}
```

This system uses the builtin `Time` resource, and a query for entities that have a `Text` and `TimeText` components. After that we just loop through them and update the text value. In practice, there should only ever be one entity that matches the query, so we could just get the first one and be done with it, but I like to loop anyway. This way, if in the future we decide to have more than one, we don't have to change the system.
>该系统使用内置的 `Time` 资源，以及具有 `Text` 和 `TimeText` 的组件的实体查询。之后，我们只需要循环遍历它们并更新文本值。在实际情况中，应该只有一个实体能匹配上查询，所以我们可以只需获取第一个实体并完成此次操作，但无论如何我还是倾向于使用循环。这样，如果将来我们决定创建多个“系统”，我们就不必修改其中的代码了。

We'll finish this file by making a plugin:
>我们通过创建一个插件来完成文件该文件：

```rust
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_time_text.system());
    }
}
```

Now, we should go into `main.rs`, to add `CameraUiBundle` to the `setup` system, and import the plugin:
>现在，进入 `main.rs`，把 `CameraUiBundle` 加到 `setup` “系统”中，并导入插件：

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;
mod consts;
mod types;
mod ui;
use ui::UIPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin) // <--- New
        .run();
}

fn setup(commands: &mut Commands) {
    let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default()) // <--- New
        .insert_resource(config);
}
```

`CameraUiBundle` is pretty similar to `Camera2dBundle`, but for UI elements. Without adding it, the text wouldn't show up. Since we've added it, we can now run the game to see our fancy fancy text on the screen:
>`CameraUiBundle` 和 `Camera2dBundle` 很类似，但对于 UI 元素。如果不显式地添加它，文本就不会显示。因为我们之前已经添加了它，现在可以运行游戏，在屏幕上可以看到华丽地文字：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_time_text.mp4)

## Adding scores
>增加得分

In this section we'll make a scoring system, so that players can see how well they did after each run. For that, let's open yet another file, `score.rs`, where we'll create a new resource that will keep track of both the score and the number of correct and failed arrows:
>在本节中，我们将创建得分系统，以便于玩家能过够在每次玩耍后看到自己地表现。为此，我们打开另一个文件 `score.rs`。在其中，我们将创建一个新的资源来记录分数以及正确的箭头和失败的箭头数量：

```rust
use crate::consts::*;

#[derive(Default)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,
}

impl ScoreResource {
    /// Increases number of corrects and adds to score
    /// 增加合适的次数值以及得分
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // Get a value from 0 to 1 according to how close the press was
        // 根据按下的按键的距离获取一个 0 到 1 的值
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // Give at least 10 points and 100 at max
        // 最少增加 10 分，最多增加不超过 100 分。
        let points = (score_multiplier * 100.).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

    /// Increases number of failures
    /// 统计失败的次数
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    // Getters

    pub fn score(&self) -> usize {
        self.score
    }
    pub fn corrects(&self) -> usize {
        self.corrects
    }
    pub fn fails(&self) -> usize {
        self.fails
    }
}
```

`ScoreResource` is a plain struct with three `usize` private fields. Instead of making the fields public, we've implemented getter and setter methods. This way, the only way to increase the number of correct arrows is by using `increase_correct`, which also increases the score, ensuring that we don't forget to do one after doing the other. We don't really need to do it this way for this game, where we'll only increase the score on once place, but for bigger projects things like this gives us confidence and peace of mind that we aren't causing unexpected bugs.
>`ScoreResource` 是一个简单的结构体，它有三个 `usize` 类型的私有字段。我们没有将字段设计成公有，而是设计成成员属性的 getter 和 setter。通过这种方式，增加合适的箭头数量的唯一方法是 `increase_correct`，它也能增加积分，我们需要保证有了这个方法后不会又编写另一个类似功能的方法。在这款游戏中，我们不需要这样，因为我们只需在一个地方增加分数，但对于其他更大的项目而言，这种做法更让我们有信心维护，它不会造成意料之外的漏洞。

Let's add this resource by going to `main.rs` and adding the following:
>我们把这个资源添加到 `main.rs`，并加上下面的引入代码：

```rust
mod score;
use score::ScoreResource;
```

And replace the `main` function with the following:
>使用下面的代码替换 `main` 函数：

```rust
fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .init_resource::<ScoreResource>() // <--- New
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .run();
}
```

With that done, we can use the resource on our systems. Namely, we'll make some adjustments to the `despawn_arrows` system in `arrows.rs`, so that it calls the increase methods when an arrow is despawned:
>完成之后，我们就能使用“系统”上的资源了。也就是说，我们对 `arrows.rs` 文件中的 `despawn_arrows` 系统做一些调整，这样，当箭头消失时，就会触发调用增加积分方法：

```rust
use crate::ScoreResource;

/// Despawns arrows when they reach the end if the correct button is clicked
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    
    // New
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);

            // New
            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // Despawn arrows after they leave the screen
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);

            // New
            score.increase_fails();
        }
    }
}
```

Easy enough change, we've added `mut score: ResMut<ScoreResource>` as a parameter to the system, so that we can edit the score, and we've added a call to `increase_correct`, which will take care of increasing the count and score, and we've also added a call to `increase_fails` when we despawn an arrow after it has left the screen, which means it was failed.
>改动很简单，我们增加 `mut score: ResMut<ScoreResource>` 作为系统的参数，以便我们可以编辑得分，我们添加了一个 `increase_correct` 方法，它会帮助我们增加积分，并且还有一个 `increase_fails` 方法，用于表示箭头离开屏幕消失时，积分增加失败。

Now, having a scoring system is all very fine and dandy, but it's a bit worthless if the player can't see how well they are doing! Let's make some changes in our UI module to also display the score:
>现在，拥有一个得分系统很不错，但如果玩家无法看到自己的表现，那就没啥价值了！我们需要在 UI 模板中加一些东西，以显示分数：

```rust
use crate::ScoreResource;

// New
struct ScoreText;
fn update_score_text(score: ChangedRes<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        text.value = format!(
            "Score: {}. Corrects: {}. Fails: {}",
            score.score(),
            score.corrects(),
            score.fails()
        );
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_time_text.system())
            .add_system(update_score_text.system()); // <--- New
    }
}
```

In `update_score_text` we're using `ChangedRes`, instead of the usual `Res`. The difference between the two is that the latter runs every frame, while systems that use `ChangedRes` only run if the resource has changed. This is cool, because the score won't change every frame, so we can save some time by only updating the text when we actually need to. It then sets the text on the entities that have a `ScoreText` component (as with `TimeText`, there should only be one, but why limit ourselves).
>在 `update_score_text` 中，我们使用 `ChangedRes`，而非普通的 `Res`。它们的区别在于后者会在每一帧都会运行一次，而 `ChangedRes` 只会在资源发生改变时才会运行。这很酷，因为分数不会再每一帧里都发生变化，所以这样可以节省一些开销，只需在需要时才更新文本。然后，它在具有 `ScoreText` 组件的实体上设置文本值（和 `TimeText` 一样，应该只有一个，但为什么要限制）。

We should also make changes in `setup_ui` to spawn a second `NodeBundle` and `TextBundle`, this time with the `ScoreText` component:
>我们还要修改 `setup_ui` 中的一些东西，在第二次产生 `NodeBundle` 和 `TextBundle` 时，使用 `ScoreText` 组件：

```rust
fn setup_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "Time: 0.0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TimeText);
        })
        
        // New
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "Score: 0. Corrects: 0. Fails: 0".to_string(),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}
```

I've decided to have this text be at the bottom left corner of the screen, but if you feel confident, as an exercise you could try changing it to be at the top left, under the time text.
>我已经打算把这个文本设置在屏幕的左下角，但如果你想练习，你可以尝试把它设置在左上角时间文本的下面。

There we go! Let's run the game to see how our hard work is paying off:
>试试吧！运行游戏，看看我们的成果如何：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_score_text.mp4)

Feel free to spice up the UI as you wish! What we've done here is just to show the basics of how to display text.
>你可以随心所欲地为 UI 增减东西！我们在这里所做地只是展示比较基础的如何展示文本。

## Loading from toml file
>从配置文件中加载数据

Currently our game's arrows are hardcoded. This has been alright for now, but we would really like it if players can make their own songs. We won't complicate ourselves by making a custom file format or any fancy things, so we'll go with the tried and tested [TOML](https://en.wikipedia.org/wiki/TOML) format, by using the [toml](https://github.com/alexcrichton/toml-rs) and [serde](https://github.com/serde-rs/serde) crates. This two crates together will help us to very easily implement TOML serialization and deserialization for our `SongConfig` struct.
>目前我们游戏中的箭头是硬编码的。目前这一切都还好，但我们希望玩家能创作自己的歌曲。我们不会通过制作自定义文件格式或任何花哨的东西使配置复杂化，所以我们将通过 [TOML](https://en.wikipedia.org/wiki/TOML) 和 [serde](https://github.com/serde-rs/serde) 库，来使用经过试用和测试的 [TOML](https://en.wikipedia.org/wiki/TOML) 格式。这两个 crate 将帮助我们非常容易地实现 `SongConfig` 结构的 TOML 序列化和反序列化。

Add the following to `Cargo.toml` file:
>向 `Cargo.toml` 文件加入以下内容：

```toml
toml = "0.5.8"
serde = "1.0.118"
serde_derive = "1.0.118"
```

We can now go into `types.rs` and start preparing our types for deserialization, by importing some things and adding the `Deserialize` and `Serialize` traits to `Directions` and `Speed`:
>我们现在可以编辑 `types.rs` 文件，并且导入准备好的类型和反序列化格式，向 `Directions` 和 `Speed` 类型中增加 `Deserialize` 和 `Serialize` trait 实现声明：

```rust
use bevy::prelude::*;

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
```

Now, we have a slight issue. Our `ArrowTime` struct has `spawn_time` as a field, but what we would like to write on the TOML file is the click time, so we can't use `ArrowTime` and `SongConfig` directly with Serde. We'll solve this by creating two new structs, `ArrowTimeToml` and `SongConfigToml`, which will be what the TOML file contains:
>现在，我们有个小问题。我们的 `ArrowTime` 结构体有 `spawn_time` 字段，但是我们想在 TOML 文件中写入点击时间，所以我们不能直接在 Serde 中使用 `ArrowTime` 和 `SongConfig`。我们会通过创建两个新结构体来解决这个问题，`ArrowTimeToml` 和 `SongConfigToml`，它们对应的数据将会被包含在 TOML 文件中：

```rust
#[derive(Deserialize, Debug)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
```

`name` will be used to store the song's display name, `filename` will be the path to the audio file, and `arrows` is the list of `ArrowTimeTomls`. `ArrowTimeToml` has the same fields as `ArrowTime`, but it has `click_time` instead of `spawn_time`.
>`name` 字段用于存储歌曲的名称，`filename` 是音频文件的路径，`arrows` 是 `ArrowTimeTomls` 列表。`ArrowTimeToml` 和 `ArrowTime` 的字段大部分一样，不同的是前者有 `click_time` ，后者没有，取而代之的是 `spawn_time`。

We'll also replace `ArrowTime::new` to instead take an `ArrowTimeToml`:
>我们也会把 `ArrowTime::new` 的入参改为 `ArrowTimeToml` 类型：

```rust
impl ArrowTime {
    fn new(arrow: &ArrowTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        Self {
            spawn_time: arrow.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow.speed,
            direction: arrow.direction,
        }
    }
}
```

Let's also add a couple fields to `SongConfig`, to keep the display name and the audio:
>让我们在 `SongConfig` 加几个字段，用来保存名称和音频：

```rust
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}
```

We keep the audio with a `Handle<AudioSource>`, which we'll load using the `AssetServer` when we transform the `SongConfigToml` to `SongConfig`.
>我们用 `Handle<AudioSource>` 保存音频，当我们把 `SongConfigToml` 转换为 `SongConfig` 时，我们会使用 `AssetServer` 加载它。

Finally, we'll change `load_config` to load a `SongConfig` from a file:
>最后，我们将修改 `load_config` 来从文件中加载 `SongConfig`：

```rust
pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // Open file and read contents
    // 打开文件并读取内容
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    // Parse using toml and Serde
    // 使用 toml 和 Serde 进行解析
    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

    // Process arrows
    // 处理箭头
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<ArrowTime>>();
    // Sort arrows by spawn_time
    // 以 spawn_time 对箭头排序
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // Load song audio and get the handle
    // 加载音频歌曲，并进行处理
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}
```

That's a few lines of code, but it's pretty straight forward: we first open the file and read it's contents, we parse it using toml's `from_str` function, then change the vector of `ArrowTimeTomls` into a vector of `ArrowTimes`, we load the song's audio using `AssetServer::load`, and then return the newly built `SongConfig`.
>只有几行代码，但时很直接：先打开文件并读取文件的内容，使用 toml 库中的 `from_str` 方法解析文件内容，然后修改 `ArrowTimeTomls` 数组为 `ArrowTimes` 数组，我们使用 `AssetServer::load` 加载歌曲音频，然后返回新构建的 `SongConfig`。

Note: `AssetServer::load` will search for the file inside the `assets` folder. `File::open` will instead search on the root folder, so we have to manually add `assets` at the beginning of the path.
>注意：`AssetServer::load` 将在 `assets` 文件夹中搜索文件。`File::open` 不会从根目录开始查找，所以我们需要手动地将 `assets` 加到路径前缀中。

We'll also need to change the `setup` system in `main.rs` to take `AssetServer` as a parameter, and we have to change the call to `load_config`:
>我们还需要修改 `main.rs` 中的 `setup` “系统”，修改 `load_config` 的调用方式，把 `AssetServer` 作为参数：

```rust
fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(config);
}
```

We'll make a new folder inside `assets` called `songs`, where we'll keep all of our songs map files and their corresponding audios. For now we'll make a placeholder file, called `test.toml`. Feel free to change the arrows to have something more elaborate, but for now we just need to have some to test:
>我们将在 `assets` 中创建一个 `songs` 文件夹，可以在其中保存所有的歌曲文件和对应的音频。现在，我们将创建一个名为 `test.toml` 的占位文件。你可以随意修改 arrows 以获得更详细的内容，现在只做一些简单测试：

```
name = "Test song"
filename = "audio.mp3"

arrows = [
    { click_time = 1.00, speed = "Slow", direction = "Up" },
    { click_time = 3.00, speed = "Slow", direction = "Down" },
    { click_time = 5.00, speed = "Fast", direction = "Left" },
    { click_time = 5.00, speed = "Slow", direction = "Right" },
    { click_time = 7.00, speed = "Slow", direction = "Up" },
    { click_time = 8.00, speed = "Medium", direction = "Up" },
    { click_time = 9.00, speed = "Slow", direction = "Left" },
    { click_time = 10.00, speed = "Slow", direction = "Right" },
    { click_time = 10.50, speed = "Medium", direction = "Right" },
    { click_time = 11.00, speed = "Slow", direction = "Up" },
    { click_time = 11.00, speed = "Slow", direction = "Down" },
]
```

Now, (legally) download your favorite song and place it inside `assets/songs`, with the name `audio.mp3`.
>现在，（合法地）下载你最喜欢的歌曲，将其放在 `assets/songs` 中，并将其命名为 `audio.mp3`。

Your assets folder should look like the following:
>你的 assets 目录应该如下方所示：

```
assets
├── fonts
│   └── FiraSans-Bold.ttf
├── images
│   ├── arrow_blue.png
│   ├── arrow_border.png
│   ├── arrow_green.png
│   └── arrow_red.png
└── songs
    ├── audio.mp3
    └── test.toml
```

Running the game now shouldn't be too different from the last section, just that the arrows you get are being loaded from an external file! That's pretty cool if you ask me :).
>现在运行游戏，应该和上一节没有太大不同，只是你得到的箭头是从外部文件加载的！如果你问我的话，我觉得相当酷 :)。

## Playing audio
>播放音频

A thing you might have noticed, is that in last section we implemented something to load the song audio, but it still doesn't play when we're playing the game. Let's implement that now! For that we'll open a new file, `audio.rs`, which will contain just one system:
>你可能注意到，在上一节中，我们做了一些加载歌曲的逻辑，但当我们玩游戏时，歌曲还是不能播放。现在，我们来实现播放！为此，我新建了一个文件，`audio.rs`，其中只含有一个“系统”：

```rust
audio.rs
use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, config: Res<SongConfig>) {
    // Song starts 3 seconds after real time
    // 歌曲将在实时的 3 秒后开始播放
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 3. && 3. <= secs {
        audio.play(config.song_audio.clone());
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(start_song.system());
    }
}
```

`start_song` uses the `Audio` resource to start playing the song after 3 seconds have passed from the beginning of the game. As you can see, we're using the same trick we used when spawning arrows.
>`start_song` 使用 `Audio` 资源，在进入游戏 3 秒后开始播放歌曲。如你所看到的，我们使用了与“生成箭头”相同的方法。

Note: We could have used a `Timer` that doesn't repeat, but this will make it more complicated later, when we make a menu to select the song. It can be a good exercise to try to rewrite it using timers though!
>注意：我们本可以复用 `Timer`，但当我们制作一个菜单来选择歌曲时，会带来一定的复杂度。何况尝试使用定时器重写，是一个很好的练习！

On `main.rs`, we should add the following:
>在 `main.rs` 中，我们添加以下内容：

```rust
// main.rs
mod audio;
use audio::AudioPlugin;
```

And in the `main` function, after all the other plugins, add `.add_plugin(AudioPlugin)`. Running the game now should result in the song starting to play as the timer starts running!
>在 `main` 函数中，在所有插件加载的最后，添加 `.add_plugin(AudioPlugin)`。现在运行游戏应该会让歌曲播放了，因为计时器在运行！

With this we have concluded implementing the core gameplay for our game. You can feel free to branch off into building your own thing on top of the base we've built here, but I recommend you stick around a bit longer, as we'll be working on making things ✨ fancier ✨.
>至此，我们完成了游戏核心实现。你可以自由地在此基础上构建你自己地东西，但我建议你在往后看看，因为我们讲致力于让游戏更加✨漂亮✨。

## 美化失败的箭头
For starters, we could improve how failed arrows look. Currently, they just fly off into the distance. We'd ideally like to give the player a little more indication that they have failed.
>首先，我们可以改进失败箭头的外观。目前，它们只是飞向远处。我们希望给玩家一些暗示，提醒他们那个箭头失败了。

What we're going to do is to have the arrows "fall off" their line after passing the target. To implement that, let's add some things to `move_arrows` in `arrows.rs`:
>我们要做的是让箭头在穿过目标区域后，“脱离”一条线。为了实现这一点，我们在 `arrows.rs` 中的 `move_arrows` 函数中加点东西：

```rust
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        // 新加代码
        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;
        }
    }
}
```

What we're doing is get the signed distance from the target to the arrow only in the `x` coordinate, and if it's positive, meaning that it's moved past the target, we substract a bit to its `y` coordinate, so it goes down. By using `time.delta_seconds() * distance_after_target`, we make the lowering factor bigger each frame, which will make it fall down in an arc. The `2.` is just a magic constant to make the arc nicer (for me), feel free to adjust it to your taste!
>我们所做的是获取目标到箭头符号的 `x` 坐标距离，如果是正的，意味着它已经移动到目标区域外，我们就在它的 `y` 坐标减去一点，这样它就会下降。通过 `time.delta_seconds() * distance_after_target`，我们让每一帧的下降因子变大，这会让箭头以弧线的形式下降。`2.` 只是一个特定的常量，使弧线更好看（我觉得是），你可以根据你自己的意愿调整它！

Here's how this looks:
>效果见下方链接的视频：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_arrows_falling.mp4)

That's good, but let's give it a bit more effect. We'll make the arrows also shrink and spin as they fall:
>很好，我们再给它加点效果。我们让箭头在下降时收缩并旋转：

```rust
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // Move the arrow down if it's past the target
            // 一旦箭头穿过目标区域，则开始下落
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;

            // Change the scale according to how far away the arrow is
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            // Rotate the arrow according to distance and speed
            // 根据距离和速度旋转箭头
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}
```

This is a bit full with magic numbers mixed with formulas I came up with after trying different things. I encourage you to try playing with it and making something else!
>这是一个充满魔力的数字和公式，我在经过多次不同的尝试得出的结论。我建议你试试其它内容！

Let's break it down a bit: first, we get a scale using a formula which decreases as the arrow moves away. We then use `max` to ensure that the scale is at least `0.2`. After that, we use [Transform::rotate](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.Transform.html#method.rotate) to rotate the arrow. For the rotation we use `Speed::multiplier`, to have the arrows spin faster if they have a faster speed. Here's how all of these look together:
>我们将其分析一下：首先，我们使用一个随着箭头移动而减小的公式来获得一个比例。然后，使用 `max` 来确保比例至少为 `0.2`。之后，我们使用 [Transform::rotate](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.Transform.html#method.rotate) 来旋转箭头。对于旋转，我们使用 `Speed::multiplier`，如果箭头的速度更快，就会旋转地更快。下面是所有这些效果组合在一起的样子：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_arrows_spinning.mp4)

Ayyy that's pretty cool! Again, feel free to improvise and add other things that make it look better. Half of the fun comes from making fancy things you enjoy looking at!
>太酷了！再次强调，你可以随时即兴发挥，添加其他逻辑，让它更加酷炫。游戏有一半的乐趣来自于制作你喜欢的花哨特效！

## Shader backgrounds
>着色器背景

Next thing we'll work is replacing the gray background. One option would be using the `ClearColor` resource to have a static color as background. [Here](https://github.com/bevyengine/bevy/blob/v0.4.0/examples/window/clear_color.rs)'s an example of how it's used. It's pretty simple, we'd just have to add `.add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))` in `main`, but this only allows us to change the background to a flat color, and we would preferably like something more animated. Shaders to the rescue!
>接下来我们要做的是替换灰色背景。选择之一是使用 `ClearColor` 资源，以静态颜色作为背景。[这里](https://github.com/bevyengine/bevy/blob/v0.4.0/examples/window/clear_color.rs)是一个使用示例。这种方式很简单，我们只需要在 `main` 函数中加上 `.add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))`，缺点是只能将背景改为一个平面颜色，我们希望看到更加生动的内容。着色器可以帮助我们！

We'll make a sprite the size of the window under everything, and we'll add a shader material. This way, we'll have a background where we can set a shader as the background.
>我们将在所有元素下面制作一个窗口大小的精灵，我们将添加着色器材料。这样我们会有一个背景，也就是设置一个着色器作为背景。

As we'll be adding some other stuff with shaders, we'll make a folder called `shaders` where we'll keep all of our things. Let's open `shaders/mod.rs`:
>当我们用着色器添加一些其他东西时，我们创建一个名为 `shaders` 的文件夹，用于存放相关文件。我们先打开 `shaders/mod.rs`：

```rust
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    window::WindowResized,
};

mod background;
use background::*;
```

Right now we've only added some imports and declared the `background` module, which we'll create now:
>现在，我们只添加了一些导入，声明了 `background` 模块，接下来就创建这个模块：

```rust
use super::*;

pub struct Background;
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    // 创建一个新的着色器管道
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    commands
        .spawn(SpriteBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(
                window.width + 10.,
                window.height + 10.,
                1.,
            )),
            ..Default::default()
        })
        .with(Background);
}
```

In this file we've added a startup system that first creates a `PipelineDescriptor`, which contains both a vertex and a fragment shader. These are added from files (which we'll create in a second) using the `include_str` macro. It then creates a `SpriteBundle` with a `RenderPipelines` component, where we pass the custom pipeline descriptor we have created. Finally, we also add a `Background` marker component.
>在这个文件中，我们添加了一个“启动系统”，它首先创建了 `PipelineDescriptor`，其中包含顶点和 fragment 着色器。这些都是用 `include_str` 宏从文件中添加进来的。然后我们会创建一个带有 `RenderPipelines` 组件的 `SpriteBundle`，并将我们创建的管道描述符传入。最后，我们添加了一个 `Background` 标记组件。

We're making use of the `WindowDescriptor` resource to access the screen width and height, so that we can set the transform correctly. There's going to be a slight issue if the player makes the window bigger, as our background will stay the same size, and that will show the gray background behind! To fix this, we'll add another small system:
>我们正在使用 `WindowDescriptor` 资源来得到屏幕宽度和高度，这样就可以进行正确的转换。如果玩家将窗口变大，会出现一个小问题，因为我们的背景大小不变，导致后面的灰色背景被显示出来！为了解决这个问题，我们添加另一个“系统”：

```rust
/// Resizes background when window is resized
pub fn update_background_size(
    mut event_reader: Local<EventReader<WindowResized>>,
    events: Res<Events<WindowResized>>,
    mut background: Query<(&mut Transform, &Background)>,
) {
    for event in event_reader.iter(&events) {
        for (mut transform, _) in background.iter_mut() {
            transform.scale = Vec3::new(event.width, event.height, 1.);
        }
    }
}
```

It listens to the [WindowResized](https://docs.rs/bevy/0.4.0/bevy/window/struct.WindowResized.html) event, which gives us the new width and height of the window every time it is resized.
>它监听 [WindowResized](https://docs.rs/bevy/0.4.0/bevy/window/struct.WindowResized.html) 事件，该事件在每次调整窗口大小时会提供新的窗口宽高。

As you may have noticed, there's a pattern of things being easy and pleasant to use in Bevy. Events are no different. To use an event, we need to add an `Event<T>` resource and a `Local<EventReader<T>>` as parameters. We can then use `EventReader::iter` by providing it the event resource, which will provide us the events that we haven't processed yet.
>正如你注意到的，在 Bevy 中有一种易于使用且优雅的模式。事件也不例外。要使用一个事件，我们需要添加一个 `Event<T>` 资源和一个 `Local<EventReader<T>>` 作为参数。然后我们就可以通过事件资源来使用 `EventReader::iter`，该事件资源将给我们提供需要处理的事件。

The actual shaders are added using Rust's `include_str` macro, which will add the contents of the file as a string. First we'll make `background.vert`:
>实际使用着色器时是使用 Rust 的 `include_str` 宏添加的，它将以字符串的形式添加文件内容。首先，我们创建 `background.vert`：

```
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 1) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
```

The only special thing we're doing here is adding `v_Uv` (the uv coordinates of the texture) as an output, so that we can use it in the fragment shader, which we'll create now in `background.frag`:
>我们在这里只需做一件特殊的事是添加 `v_Uv`（纹理的 uv 坐标）作为输出，这样，我们就可以在 fragment 着色器中使用它，现在我们在 `background.frag` 中创建它：

```
// shaders/background.frag
#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vec4(v_Uv, 0.1, 1.0);
}
```

In this shader we only return a simple color based on the uv coordinates of the background.
>在这个着色器中，我们只返回基于背景的 uv 坐标的简单颜色。

We now need to register these systems we have created. Let's add `ShaderPlugin` in `shaders/mod.rs`:
>我们现在需要注册这些创建的“系统”。我们在 `shaders/mod.rs` 中添加 `ShaderPlugin`：

```rust
// shaders/mod.rs
pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_background.system())
            .add_system(update_background_size.system());
    }
}
```

We can now go into `main.rs` and import it:
>现在我们可以在 `main.rs` 中导入它：

```rust
mod shaders;
use shaders::ShadersPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin) // <--- New
        .run();
}
```

Running the game now will show you this:
>运行游戏你可以看到下方链接视频中展示的效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_simple_background.mp4)

## Using time in shaders
>使用时间着色器

We have some kind of fancy background going on, cool! Ideally though, we'd like to have it change through time.
>我们有一些奇特的场景，酷！理想情况下，我们希望游戏背景随着时间有一些变化。

Bevy doesn't (at least not for now) add the time nor the resolution into shaders as an input, so we'll have to manually add them ourselves. Hopefully this will be improved soon.
>Bevy 没有（至少现在没有）添加时间和分辨率到着色器中作为输入，所以我们将不得不手动添加它们。希望这点能在 Bevy 中很快得到改善。

Let's open `shaders/mod.rs` again and add the following code:
>我们再次打开 `shaders/mod.rs`文件，并增加以下代码：

```rust
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
/// Resource that will be passed to shaders
/// 资源将传递给着色器
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

/// Updates time in ShaderInputs every frame
/// 在每一帧中，更新 ShaderInputs 中的时间
fn update_time(time: Res<Time>, mut nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    for mut node in nodes.iter_mut() {
        node.time = time as f32;
    }
}

/// Updates resolution in ShaderInputs if window size changes
/// 如果窗口大小发生改变，更新 ShaderInputs 的分辨率
fn update_resolution(
    mut event_reader: Local<EventReader<WindowResized>>,
    events: Res<Events<WindowResized>>,
    mut background: Query<&mut ShaderInputs>,
) {
    for event in event_reader.iter(&events) {
        for mut node in background.iter_mut() {
            node.resolution = Vec2::new(event.width / event.height, 1.);
        }
    }
}

/// Adds ShaderInputs as an edge in the render graph
/// 在渲染图形时，添加 ShaderInputs 作为一个 edge
fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}
```

We're making a new struct called `ShaderInputs`, which we add as a render graph edge in `setup_render_graph`, which we'll add as a startup system. `update_time` and `update_resolution` are two systems that take care of updating the values for each entity that has the `ShaderInputs` component. Notice how on `update_resolution` we're listening to the `WindowResized` event instead of updating every frame.
>我们正在创建一个新的 `ShaderInputs` 结构体，将其作为渲染图形边添加到 `setup_render_graph` 中，并将其加到“启动系统”中。`update_time` 和 `update_resolution` 是两个负责更新 `ShaderInputs` 组件值的系统。注意在 `update_resolution` 中我们是如何监听 `WindowResized` 事件，而非更新每一帧。

Now, replace `ShaderPlugin` with the following, to add all of these systems and assets:
>现在，用以下代码替换 `ShaderPlugin` 中的实现，添加所有系统和资源：

```rust
pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ShaderInputs>() // <--- New
            .add_startup_system(setup_render_graph.system()) // <--- New
            .add_system(update_time.system()) // <--- New
            .add_system(update_resolution.system()) // <--- New
            .add_startup_system(setup_background.system())
            .add_system(update_background_size.system());
    }
}
```

We're now going to add `ShaderInputs` as a component to the background entity we previously created, providing the initial values:
>我们现在要向之前创建的背景实体添加 `ShaderInputs` 组件，提供初始值：

```rust
// shaders/background.rs
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    // 创建新的着色器管道
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    commands
        .spawn(SpriteBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(
                window.width + 10.,
                window.height + 10.,
                1.,
            )),
            ..Default::default()
        })
        .with(Background)
        // New
        .with(ShaderInputs {
            time: 0.,
            resolution: Vec2::new(window.width / window.height, 1.),
        });
}
```

Those values are now available to us on the shader, by adding a couple things:
>这些值通过添加一些东西后，现在可以在着色器上使用了：

```
// shaders/background.frag
#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

// New
layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
// New
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};

void main() {
    o_Target = vec4(v_Uv, abs(sin(time)), 1.0);
}
```

Basically, for each of the fields in `ShaderInputs`, we have to add a uniform, with increasing values in `binding` and a name like `ShaderInputs_$name`, where `$name` is the field name. We can now use the variables inside the shader!
>基本上，我们必须对 `ShaderInputs` 的每个字段增加 uniform，它包含 `binding` 对应增加的值，以及形如 `ShaderInputs_$name` 的名字，其中的 `$name` 是字段名。现在我们可以使用着色器内部的变量了！

Here's how this looks like:
>现在看起来应该如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_background_with_time.mp4)

Personally, I've settled on the following shader as a background:
>就个人而言，我选择了以下配置的着色器作为背景：

```
#version 450

#define TWO_PI 6.28318530718

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};

vec3 hsb2rgb(in vec3 c){
    vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
                             6.0)-3.0)-1.0,
                     0.0,
                     1.0 );
    rgb = rgb*rgb*(3.0-2.0*rgb);
    return c.z * mix( vec3(1.0), rgb, c.y);
}

float wave_sin(in float x) {
    float amplitude = 0.5;
    float frequency = 1.0;
    float y = sin(x * frequency);
    float t = 0.01*(-time*50.0);
    y += sin(x * frequency * 2.1 + t)*4.5;
    y += sin(x * frequency * 1.72 + t*1.121)*4.0;
    y += sin(x * frequency * 2.221 + t*0.437)*5.0;
    y += sin(x * frequency * 3.1122+ t*4.269)*2.5;
    y *= amplitude*0.06;
    return y;
}
float wave_cos(in float x) {
    float amplitude = 0.5;
    float frequency = 2.0;
    float y = cos(x * frequency);
    float t = 0.01*(-time*30.0);
    y += cos(x * frequency * 2.1 + t)*4.5;
    y += cos(x * frequency * 1.72 + t*1.121)*4.0;
    y += cos(x * frequency * 2.221 + t*0.437)*5.0;
    y += cos(x * frequency * 3.1122+ t*4.269)*2.5;
    y *= amplitude*0.06;
    return y;
}
vec2 wave(in vec2 v) {
    return vec2(wave_sin(v.x), wave_cos(v.y));
}

void main() {
    vec2 uv = wave(v_Uv);
    vec3 color = hsb2rgb(vec3(uv.x + sin(uv.y), 0.7, 1.0));

    o_Target = vec4(color,1.0);
}
```

It just shifts the colors around and makes pretty waves, which looks like the following:
>它只是移动周围的颜色，产生好看的波浪，效果如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_fancy_background.mp4)

Now it's your turn to play around with it and find something you like. If you don't feel too confident with shaders, you can try making slight modifications to the one above, or you can go to [Shadertoy](https://www.shadertoy.com/) and find inspiration from there. For example, the following is a [shader](https://www.shadertoy.com/view/XsXXDn) by Danilo Guanabara, translated from Shadertoy:
>现在轮到你玩它了，找到你喜欢的东西。如果你不太理解着色器，你可以尝试对上面的着色器做一些小修改，你也可以去 [Shadertoy](https://www.shadertoy.com/) 查找一些资料。例如，下面是一个 [shader](https://www.shadertoy.com/view/XsXXDn) 配置，它由 Danilo Guanabara 转换自 Shadertoy：

```
// shaders/background.frag
#version 450

// Creation, by Silexars (Danilo Guanabara)
// From https://www.shadertoy.com/view/XsXXDn

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};

void main() {
    vec3 c;
    vec2 r = resolution;
    float l,z=time;
    for(int i=0;i<3;i++) {
        vec2 uv,p = v_Uv; // / r;
        uv = p;
        p -= 0.5;
        p.x *= r.x/r.y;
        z += 0.07;
        l = length(p);
        uv += p/l*(sin(z)+1.)*abs(sin(l*9.0-z*2.0));
        c[i] = (0.01)/length(abs(mod(uv,1.0)-0.5));
    }
    o_Target = vec4(c/l,time);
}
```

And here's the result:
>效果如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_creation_background.mp4)

## Fancy click animation
>美化点击动画

We've previously added a fancier animation for when an arrow is failed, but we still do nothing when an arrow is correctly clicked. It just disappears, which is slightly disappointing. Let's work on improving that.
>我们之前已经为失败的箭头添加了有趣动画，但当成功命中箭头时，我们啥也没做。它就这样消失了，这有点让人失望。我们将这一点进行改进。

We're going to have four different sprites, each with a shader material, under each of the target arrows. Then, we'll make it so each time an arrow is correctly clicked, the shader of the corresponding sprite starts the animation, which will last some time and then disappear.
>我们将有四个不同的“精灵”，每个精灵在每个目标区域箭头下都有一个着色器。然后，每当正确命中箭头时，相应的精灵下的着色器就会启动动画，动画持续一段时间后，再消失。

Note: The way this is going to be implemented is a bit more complex than what we could technically do, but it allows me to show some more stuff. An easier way to implement this would be to create a sprite each time an arrow is correctly clicked, and then removing it after some seconds.
>注意：这个如果用技术实现会比较复杂，但这样可以展示很多东西。实现这一点有个捷径是在每次正确点击箭头时创建一个精灵，然后几秒钟后删除掉。

We'll open a file named `shaders/target_arrows.rs`, where we'll add a component for these sprites (which I'm calling "target arrow sparkles"), which only keeps the direction of the target arrow it's associated to:
>我们将打开 `shaders/target_arrows.rs` 文件。我们将为这些精灵添加一个组件（我把它叫做“普通目标箭头”），它只是指示目标箭头的方向：

```rust
pub struct TargetArrowSparkle {
    direction: Directions,
}
```

We'll also add another edge to the render graph, with another struct we want to pass to the shader as parameter. This will keep the last time that there was a correct arrow, and how many points it was worth:
>我们再添加另一条边到渲染图中，并将另一个结构体作为参数传递给着色器。这将保留最后一次正确命中箭头的时间，以及对应得分：

```rust
// shaders/target_arrows.rs
#[derive(RenderResources, TypeUuid)]
#[uuid = "c9400817-b3a3-4baa-8bfa-0320b9b87b17"]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}
```

Notice that when we add the `TimeSinceLastCorrect` component to the target arrow sparkles, each one will have it's own values that are not shared, so we can set them individually.
>请注意，当我们向目标箭头添加 `TimeSinceLastCorrect` 组件时，每个组件都有自己得值，这些值是不共享的，所以我们需要单独设置它们。

Now, let's add a startup system to create the sprites:
>现在，我们添加一个“启动系统”用于创建精灵：

```rust
// shaders/target_arrows.rs
use super::*;
use crate::consts::*;
use crate::types::Directions::{self, *};

pub fn setup_target_arrows(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    // 创建一个新的着色器管道
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("target_arrows.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("target_arrows.frag"),
        ))),
    }));

    // Add TimeSinceLastCorrect to the render graph
    // 把 TimeSinceLastCorrect 加到渲染图中
    render_graph.add_system_node(
        "last_time",
        RenderResourcesNode::<TimeSinceLastCorrect>::new(true),
    );
    render_graph
        .add_node_edge("last_time", base::node::MAIN_PASS)
        .unwrap();

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        // Different z values so they don't overlap
        // z 值不同，所以它们不会重叠
        let z = match direction {
            Up => 0.3,
            Down => 0.4,
            Left => 0.5,
            Right => 0.6,
        };

        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), z));
        transform.scale = Vec3::new(300., 300., 1.);
        commands
            .spawn(SpriteBundle {
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle.clone(),
                )]),
                transform,
                visible: Visible {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(TargetArrowSparkle {
                direction: *direction,
            })
            .with(TimeSinceLastCorrect {
                last_time: 3.,
                points: 0.5,
            })
            .with(ShaderInputs {
                time: 0.,
                resolution: Vec2::new(window.width / window.height, 1.),
            });
    }
}
```

This system is like a mix of `setup_target_arrows`, `setup_render_graph`, and `setup_background`. We first create a `PipelineDescriptor`, then add `TimeSinceLastCorrect` as a render graph edge, and finally we make an array of all the directions and iterate over it, creating the 4 sprite bundles, adding `TargetArrowSparkle`, `TimeSinceLastCorrect`, and `ShaderInputs` as components.
>这个系统就像是 `setup_target_arrows`，`setup_render_graph` 和 `setup_background` 的混合体。我们首先创建一个 `PipelineDescriptor`，然后添加 `TimeSinceLastCorrect` 作为渲染图的边缘，最后我们创建一个存放所有方向的数组，然后迭代它，创建 4 个精灵组，并添加 `TargetArrowSparkle`，`TimeSinceLastCorrect` 和 `ShaderInputs` 组件。

We've set `last_time` to 3 seconds, to test. This will make it so that when the time reaches three, the animation starts. When we have everything properly set up we'll change it to a negative value, as we don't want the animation going off until an arrow is correctly clicked.
>我们把 `last_time` 设为 3 秒进行测试。这样，当时间达到三秒时，动画就开始了。当我们设置好所有内容后，我们会将其更改为负值，因为我们希望箭头在被正确点击时触发。

We also need to create new files for the shaders:
>我们还需要为这个着色器创建新文件：

```
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 1) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
```

The vertex shader is exactly the same as `shaders/background.vert`. The more interesting one is `shaders/target_arrows.frag`:
>vertex 着色器的实现基本上和 `shaders/background.vert` 一样。更有意思的是 `shaders/target_arrows.frag`：

```
# shaders/target_arrows.frag
#version 450

#define TWO_PI 6.28318530718

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};
layout(set = 3, binding = 0) uniform TimeSinceLastCorrect_last_time {
    float last_time;
};
layout(set = 3, binding = 1) uniform TimeSinceLastCorrect_points {
    float points;
};

float interval(in float a, in float b, in float val) {
    return step(a, val) * smoothstep(1.0 - b - 0.1, 1.0 - b, 1. - val);
}

float circle(in vec2 uv, in float _radius){
    vec2 dist = uv - vec2(0.5);
    return 1.0 - smoothstep(_radius - (_radius * 0.01),
                            _radius + (_radius * 0.01),
                            dot(dist, dist) * 4.0);
}

float smoothcircle(in vec2 _st, in float s){
    vec2 dist = _st-vec2(0.5);
    return 4. * dot(dist,dist) / (s);
}

void main() {
    // 0. when the circle shouldn't be shown
    float alpha = interval(last_time, last_time + 0.6, time);

    // Circle radius
    float radius = time - last_time;
    // 0. for not in circle, 1. for circle
    // float circle = circle(v_Uv, radius) * (1. - circle(v_Uv, radius - 0.1));
    float circle = smoothcircle(v_Uv, radius) * smoothcircle(v_Uv, radius) * circle(v_Uv, radius);

    // rgb(92, 175, 29);
    vec3 colorMin = vec3(0.36078431373,0.6862745098,0.1137254902);
    // rgb(255, 255, 6);
    vec3 colorMax = vec3(1.,1.,0.02352941176);

    // Get color according to points
    vec3 color = mix(colorMin, colorMax, points);

    o_Target = vec4(color * circle, circle * alpha);
}
```

This shader is a bit more complicated, but in short what it does is create a circle with a radius that increases with time. The circle only lasts `0.6` seconds after `last_time`. We have used a layout with set 3 to add `TimeSinceLastCorrect`, and as with `ShaderInputs`, the bindings increase in value for each of the fields. The color of the circle changes depending on the points.
>这个着色器有点复杂，但简而言之，它的作用是创建一个半径随时间增加的圆。圆圈在 `last_time` 后存在 `0.6` 秒。我们使用一个值为 3 的设置来添加 `TimeSinceLastCorrect`，并且和 `ShaderInputs` 一样，每个字段的绑定值都会增加。圆形的颜色根据点的不同而有所变化。

We also need to add `setup_target_arrows` to `ShaderPlugin`:
>我们还需要把 `setup_target_arrows` 加到 `ShaderPlugin` 中：

```rust
// shaders/mod.rs
mod target_arrows;
use target_arrows::*;

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ShaderInputs>()
            .add_asset::<TimeSinceLastCorrect>()
            .add_startup_system(setup_render_graph.system())
            .add_system(update_time.system())
            .add_system(update_resolution.system())
            .add_startup_system(setup_background.system())
            .add_system(update_background_size.system())
            .add_startup_system(setup_target_arrows.system()); // <--- New
    }
}
```

Running the game now will result in the following:
>现在运行游戏，将看到如下面链接视频所展示的效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_half_done_fancy_clicking.mp4)

As you can see, right when the song starts, at 3 seconds, all of the circles start growing, and a bit after half a second they disappear. This is great news, it means that both the shader and the time are working! We're still missing something to update the value though, so let's add a system to update the `last_time` value when an arrow is correctly clicked. Before that, let's set the default value to be something negative:
>如你所看到的，就在歌曲开始后，第 3 秒时，所有的圆圈开始变大，约过半秒后它们就消失了。太好了，这意味这着色器和定时器都正常工作了！我们仍然缺少一些东西来更新一些值，所以我们添加一个“系统”，用于当箭头被正确的按下时，更新 `last_time` 值。在此之前，我们使其默认值为负的：

```rust
// shaders/target_arrows.rs
.with(TimeSinceLastCorrect {
    last_time: -10.,
    points: 0.,
})
```

If you run the game now, the circles shouldn't appear at all.
>现在如果你运行这个游戏，圆圈就不会出现了。

Previously, we've seen how to listen to events, but we still haven't looked at the other side of the coin. We'll be working with sending them now. We'll make an event that is sent when an arrow is correctly clicked. We'll send this event from inside the `despawn_arrows` system, in `arrows.rs`:
>之前，我们已经看到了如何侦听事件，但我们仍然没有看到硬币的另一面。我们现在就准备探索一下。我们将创建一个正确点击箭头时发生的事件。我们在 `arrows.rs` 文件中的 `despawn_arrows` 中产生这个事件：

```rust
// arrows.rs
/// 事件结构体
pub struct CorrectArrowEvent {
    pub direction: Directions,
    pub points: usize,
}

/// Despawns arrows when they reach the end if the correct button is clicked
/// 当他们到达目标区域时，正确点击按钮，箭头就会消失
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
    mut correct_arrow_events: ResMut<Events<CorrectArrowEvent>>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);

            let points = score.increase_correct(TARGET_POSITION - pos);

            // New
            
            // Send event
            // 发送事件
            correct_arrow_events.send(CorrectArrowEvent {
                direction: arrow.direction,
                points,
            });
        }

        // Despawn arrows after they leave the screen
        // 当箭头离开屏幕时消除它们
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
            score.increase_fails();
        }
    }
}
```

The first thing we've done is create a new `CorrectArrowEvent` struct, which will be our event. To `despawn_arrows`, we've added a `ResMut<Events<CorrectArrowEvent>>` parameter, so that we can `send` events through it, using the send method. To send an event we need to pass a `CorrectArrowEvent` struct, with the direction of the arrow and the points that the player got.
>我们首先要做的是创建一个新的 `CorrectArrowEvent` 结构体，它用来表示我们的事件。对于 `despawn_arrows`，我们添加了 `ResMut<Events<CorrectArrowEvent>>` 参数，这样我们就能通过 `send` 方法发送事件。为了发送一个事件，我们需要传入一个 `CorrectArrowEvent` 结构体，它携带箭头的方向以及玩家的得分。

We now need to add `.init_resource::<Events<CorrectArrowEvent>>()` to `ArrowsPlugin`, and we're ready to go. Easy, right?
>现在我们需要把 `.init_resource::<Events<CorrectArrowEvent>>()` 添加到 `ArrowsPlugin`，我们已经准备好了。很简单，对吧？

We're now going to add a system in `shaders/target_arrows.rs` which takes care of updating `last_time` in the correct target arrow sparkles:
>现在我们要在 `shaders/target_arrows.rs` 中添加一个“系统”，它负责更新“目标区域箭头”中的 `last_time`：

```rust
// shaders/target_arrows.rs
pub fn correct_arrow_event_listener(
    time: Res<Time>,
    mut correct_event_reader: Local<EventReader<CorrectArrowEvent>>,
    correct_events: Res<Events<CorrectArrowEvent>>,
    mut query: Query<(&TargetArrowSparkle, &mut TimeSinceLastCorrect)>,
) {
    for event in correct_event_reader.iter(&correct_events) {
        for (arrow, mut last_correct) in query.iter_mut() {
            if arrow.direction == event.direction {
                last_correct.last_time = time.seconds_since_startup() as f32;
                last_correct.points = event.points as f32 / 100.;
            }
        }
    }
}
```

It does this by listening to the event, looking for the target arrow sprite that is associated to that direction, and updating the `last_time` and `points` on in.
>它通过监听事件，寻找与目标方向相关的箭头精灵，并更新其中的 `last_time` 和 `points` 值。

Add this last system to `ShaderPlugin`, `.add_system(correct_arrow_event_listener.system())`. If you run the game now, you'll get the circles when you correctly click an arrow:
>把最后一个“系统”加到 `ShaderPlugin`，`.add_system(correct_arrow_event_listener.system())`。现在如果你运行游戏，当你正确点击箭头时，就会看到圆圈效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_target_arrow_circles.mp4)

That's all of the shader-ing we'll do in this game. As always, feel free to change things up, add more effects, and experiment!
>这就是这个游戏中我们要做的所有着色工作。和以往一样，你可以随便修改代码，添加更多效果，进行实验！

## Adding states
>增加状态

In the next sections we'll work on making a very simple song select menu. For that we'll be working with States, which will require some changes all over the place. To make a State, we need to create a new enum, and add it as a resource wrapped in a [State](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.State.html). Then, we can assign each system to a specific state, using `on_state_update`, `on_state_enter`, and `on_state_exit`.
>在下一节，我们将制作一个非常简单的歌曲选择菜单。为此，我们将在一些状态值上下手，这就需要修改一些地方。为了创建一个状态，我们需要新建一个新的枚举，并将其包装成 [State](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.State.html) 的资源加到游戏代码中。然后，我们可以使用 `on_state_update`，`on_state_enter` 和 `on_state_exit` 等方法为每个系统分配特定的状态。

Let's start working on this. First, let's open `consts.rs` and add our state enum:
>我们开始吧。首先，打开 `consts.rs`，添加 state 枚举：

```rust
/// Stage for our systems
pub const APP_STATE_STAGE: &str = "app_state_stage";

/// States
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
    MakeMap,
}
```

`AppState` will represent the three states our game has: song select menu, the game, and the (still unimplemented) map maker mode.
>`AppState` 将代表我们游戏的三个状态：歌曲选择菜单，游戏和（尚未实现的）地图制作模式。

We're also adding a string for the stage we'll use for our systems. Now we'll go into `main.rs` and add both the `State` resource, and our new stage after Update:
>我们，还添加了一个字符串用于我们的系统。现在我们进入 `main.rs` 中，添加 `State` 以及更新后的新阶段两个资源：

```rust
// main.rs
use crate::consts::*;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu)) // <--- New
        .add_stage_after( // <--- New
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .run();
}
```

Running the game now doesn't change anything at all, as our systems are still being added normally. To change that, we'll start by changing `ArrowsPlugin` in `arrows.rs`:
>现在游戏不会有任何变化，因为我们的“系统”仍然以普通的方式加入。为了改变这一点，我们将从修改 `arrows.rs` 中的 `ArrowsPlugin` 入手：

```rust
// arrows.rs
pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .init_resource::<Events<CorrectArrowEvent>>()
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::Game,
                setup_target_arrows.system(),
            )
            .on_state_update(APP_STATE_STAGE, AppState::Game, spawn_arrows.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, move_arrows.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, despawn_arrows.system());
    }
}
```

We have to replace `add_startup_system` for `on_stage_enter`, and `add_system` for `on_stage_update`. To these functions, we have to pass the stage and the state on which we want the system to run. As we want all of these running on the `Game` state, that's the one we use.
>我们必须把 `add_startup_system`替换为 `on_stage_enter`，将 `add_system` 替换为 `on_stage_update`。对于这些函数，我们必须传入“系统”运行的阶段和状态。因为我们想要所有这些运行在 `Game` 状态，就是我们使用的那个。

Let's now go to `ui.rs`:
>现在我们看看 `ui.rs`：

```rust
// ui.rs
use crate::consts::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::Game, setup_ui.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_time_text.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_score_text.system());
    }
}
```

In `audio.rs`:

```rust
// audio.rs
use crate::consts::*;

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(APP_STATE_STAGE, AppState::Game, start_song.system());
    }
}
```

We've changed all of the game related systems to be associated with the `Game` state, so if you run the game now, nothing will happen, apart from seeing the animated background, because we start in the `Menu` state, but we have no systems associated with it yet.
>我们已经修改了所有与 `Game` 状态相关的“系统”，所以如果你现在运行游戏，除了看到动画背景外，什么也不会发生，因为我们要从 `Menu` 开始，但是我们还没有相关的“系统”。

## Adding a basic menu
>添加基础菜单

We'll make an actual menu with buttons now, that allows us to either select a song or to enter map maker mode. We'll keep it all in a new file, `menu.rs`. We'll start by making a resource to keep the materials:
>我们现在将制作一个带有按钮的菜单，它可以让我们选择一首歌曲或进入游戏地图制作模式。我们将它保存在一个新的文件 `menu.rs` 中。我们新建一个资源来保存对应的素材：

```rust
use crate::consts::*;
use bevy::prelude::*;

struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();

        ButtonMaterials {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}
```

This is pretty standard. Next we'll make a system that will create the menu elements:
>这看起来很标准。接下来，我们将创建一个“系统”来构建菜单元素。

```rust
// menu.rs
struct MenuUI;
fn setup_menu(commands: &mut Commands, button_materials: Res<ButtonMaterials>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: button_materials.none.clone(),
            ..Default::default()
        })
        .with(MenuUI)
        .with_children(|parent| {
            // Spawn a new button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: "Play".to_string(),
                            font: button_materials.font.clone(),
                            style: TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                });
        });
}
```

This looks very similar to `setup_ui` in `ui.rs`, but the structure looks like `NodeBundle > ButtonBundle > TextBundle`.
>这看起来非常类似于 `ui.rs` 中的 `setup_ui`。但结构类似于 `NodeBundle > ButtonBundle > TextBundle`。

We'll also make a system that removes all of the buttons, so that we can run in when we leave the `Menu` state. If we didn't, the buttons would still stay over the screen on `Game` mode.
>我们还要创建一个删除所有按钮的系统，这样我们就可以在离开菜单时运行它。如果不这样做，菜单按钮会一直停留在游戏屏幕上。

```rust
// menu.rs
fn despawn_menu(commands: &mut Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}
```

Let's also make a plugin for these systems:
>给这个系统实现插件：

```rust
// menu.rs
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(APP_STATE_STAGE, AppState::Menu, setup_menu.system())
            .on_state_exit(APP_STATE_STAGE, AppState::Menu, despawn_menu.system());
    }
}
```

And let's add it in `main.rs`, by importing the module and adding `.add_plugin(MenuPlugin)` on `main`:
>把它添加到 `main.rs` 中，导入它并在 `main` 函数中增加 `.add_plugin(MenuPlugin)` 调用：

```rust
// main.rs
mod menu;
use menu::MenuPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin) // <--- New
        .run();
}


fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
```

We also change `setup` to no longer add the `SongConfig` resource, as we'll add it when the player clicks a button to select the song.
>我们还要更改 `setup`，不再添加 `SongConfig` 资源，因为我们会在玩家点击按钮选择歌曲时添加它。

Running the game now shows the following Play button:
>现在运行游戏会显示下面这样的按钮：

![](https://caballerocoll.com/images/rhythm_basic_menu.png)

Currently, clicking and hovering over the button doesn't do anything, so let's work on making the menu more reactive. First, we'll add a system that changes the color according to the interaction state of the button:
>目前，单机按钮并将鼠标悬停在按钮上会发现按钮什么也没有干，所以我们需要让菜单能根据需要有所反应。首先，我们将添加一个系统，根据按钮的交互改变颜色：

```rust
// menu.rs
fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
```

Here we're using the `Interaction` component that comes with the `ButtonBundle`. It has three different variants, `Clicked`, `Hovered`, and `None`. Each correspond to: clicking on the button, hovering on the button, and not doing anything. We're matching on the value for each of the buttons, to change their material accordingly. Add the game to out `MenuPlugin` and run the game to see how the button changes color as you hover, click, or remove the mouse.
>这里我们使用的是 `Interaction` 组件，它和 `ButtonBundle` 一起。它有三个不同的变体，`Clicked`，`Hovered` 和 `None`。分别表示：单机按钮，悬停在按钮上，不做任何事。我们将匹配按钮的所有可能的值，从而做出不同的反应。将 `MenuPlugin` 加到游戏中，运行游戏，看看鼠标悬停、点击或移开时按钮的颜色是如何变化的。

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_button_interactions.mp4)

## Improving our menu
>优化菜单

We still need two things: the menu to show the list of songs inside the folder, and the buttons to actually start the game. Let's start with the first of those, by adding another method in `menu.rs`:
>我们还需要两个东西：在文件夹中显示歌曲列表菜单，以及正式开始游戏的按钮。我们从第一点开始，在 `menu.rs` 中增加一个方法：

```rust
// menu.rs
use std::fs::read_dir;

pub fn get_songs() -> Vec<String> {
    let paths = read_dir("assets/songs").unwrap();

    let mut vec = vec![];
    for path in paths {
        let path = path.unwrap().path();

        if "toml" == path.as_path().extension().unwrap() {
            vec.push(
                path.as_path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    vec
}
```

This function uses [`read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) to get the files inside the `songs` folder, and adds the ones ended in `.toml` to a vector.
>这个函数使用 [`read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) 获取 `songs` 目录中的文件，并将 `.toml` 后缀文件路径追加到数组中。

We can now call this function from inside `setup_menu` to add a button for each one of the files we get from `get_songs`. First, we'll make an enum component to add to our buttons:
>现在我们可以从 `setup_menu` 内部调用这个函数，来为 `get_songs` 得到的每个文件增加按钮。首先，我们创建一个枚举组件加到按钮中：

```rust
// menu.rs
enum MenuButton {
    MakeMap,
    PlaySong(String),
}
impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::MakeMap => "Make map".to_string(),
            Self::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}
```

The first variant of the enum, `MakeMap`, will enter the map maker mode (when we implement it). The other variant, `PlaySong`, will be used for each of the buttons that will start the game with a certain song.
>枚举的第一个变体 `MakeMap` 用于进入地图制作模式（如果实现了）。另一个变体 `PlaySong` 用于开始特定的歌曲游戏。

```rust
// menu.rs
fn setup_menu(commands: &mut Commands, button_materials: Res<ButtonMaterials>) {
    // Make list of buttons
    let mut buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();
    buttons.push(MenuButton::MakeMap);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: button_materials.none.clone(),
            ..Default::default()
        })
        .with(MenuUI)
        .with_children(|parent| {
            // Add all of the buttons as children
            // 将所有按钮以子按钮的方式加入
            for button in buttons {
                // Spawn a new button
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                value: button.name(),
                                font: button_materials.font.clone(),
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        });
                    })
                    .with(button);
            }
        });
}
```

We've replaced the content inside `with_children` to loop over the button list, to create each of the buttons.
>我们已替换了 `with_children` 的内容，来循环遍历按钮列表，从而创建按钮。

Note: The way we've setup the buttons is very naive, so if you have a lot of buttons to display, it's going to look weird! A system for scrolling the buttons or anything to improve how the menu looks is left as an exercise for the reader.
>注意：我们设置按钮的方式有点菜，所以如果你有很多按钮显示的话，它会看起来很奇怪！添加一个滚动条或者其他改善方式就留给读者作为练习了。

Here's the result:
>效果如下图所示：

![](https://caballerocoll.com/images/rhythm_menu_with_correct_buttons.png)

Let's now work on making the buttons usable. For that, we'll add another system that will listen to clicks:
>现在我们要让按钮可用。为此，我们添加另一个“系统”来监听点击事件：

```rust
// menu.rs
use crate::types::load_config;

fn button_press_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Mutated<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        // Check if button has been clicked this frame
        // 在这一帧中检测按钮是否被点击
        if *interaction == Interaction::Clicked {
            match button {
                // If it's the map maker button, change the state
                // 如果地图制作按钮被点击，改变状态
                MenuButton::MakeMap => state
                    .set_next(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                // If it's a play song button, load the config, insert it, and change state
                // 如果它是一个播放歌曲按钮，加载对应配置，加入，然后改变状态
                MenuButton::PlaySong(song) => {
                    let config = load_config(&*format!("{}.toml", song), &asset_server);
                    commands.insert_resource(config);
                    state
                        .set_next(AppState::Game)
                        .expect("Couldn't switch state to Game")
                }
            };
        }
    }
}
```

In this system, we loop through each button, and check if they're in a Clicked state. If they are, we match on the type of button, and act accordingly. For `MakeMap`, we just change the state, using the `set_next` method. For `PlaySong`, we use the `load_config` function we've created to load the `SongConfig` for the selected song, and we add it using `insert_resource`, before changing the state to `Game`.
>在这个系统中，我们循环遍历每个按钮，并检查它们是否处于点击状态。如果是，我们会匹配按钮的类型，执行相应的逻辑。对于 `MakeMap`，我们只需使用 `set_next` 改变状态。对于 `PlaySong`，用我们创建的 `SongConfig` 函数来加载选定歌曲的 `SongConfig`，在将状态更改为 `Game` 之前，我们使用 `insert_resource` 添加歌曲。

Finally, we should add a this system to `MenuPlugin`, set to run on update with the `Menu` state:
>最后，我们应该把这个系统添加到 `MenuPlugin`，设置为 `Menu` 状态运行：

```rust
// menu.rs
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(APP_STATE_STAGE, AppState::Menu, setup_menu.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::Menu,
                button_color_system.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::Menu,
                button_press_system.system(),
            )
            .on_state_exit(APP_STATE_STAGE, AppState::Menu, despawn_menu.system());
    
}
```

Running the game now, we'll see that the button works correctly, and starts the game:
>现在运行游戏，我们会看到按钮正常工作，开始游戏：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_menu_changing_state.mp4)

But there's a big issue! The arrows aren't appearing, and the time is already running when we get to the game! Since we're using `time_since_startup` to check when to spawn the arrows, when we enter the `Game` state, the value is already past the first arrow's spawn time, so it doesn't appear, and none of the others do either. To fix that, we'll make a wrapper over time, so that we can reset it when we enter `Game` mode.
>但有个大问题！当我们开始游戏时，时间在跑了，箭头却没有显示！因为我们使用 `time_since_startup` 来检查合适生成箭头，当我们进入 `Game` 状态时，值已经过了第一个箭头的生成时间，所以不会出现，其它箭头也不会出现。为了解决这个问题，我们将在后面制作一个包装器，这样我们就可以在进入游戏模式时重置它。

## Wrapping time
>时间系统封装

Our time wrapper will be very similar to Bevy's normal implementation of the Time resource, but it will have a system that resets time when we enter the `Game` or `MakeMap` states. It will look a bit bad to copy a all of the code just to be able to change something, but this would allow us to do other time related things in the future, like for example pausing. It's also a good excuse to look at some of Bevy's internal code.
>我们的时间包装器非常类似于 Bevy 的时间资源实现，不同的是它需要在我们进入 `Game` 和 `MakeMap` 状态时重置时间系统。复制所有代码只是为了改善一些糟糕的东西，但这会让我们在未来做其他工作时带来方便，比如暂停。这也是一个了解 Bevy 源码的好机会。

Also, by having both a the normal time resource and our wrapped version, it allows us to use the normal non-resetting time for things, and the controlled time for some others. For example, we'll keep using the normal time for the background, as we want it to work during all the states.
>此外，通过同时拥有一个正常的时间资源和我们自己包装的版本，可以让我们使用正常的非重置时间，以及其他需要控制时间的场景。例如，我们要继续为游戏背景使用正常时间，因为我们希望它在所有状态下都能工作。

Let's open a new file, `time.rs`:
>打开一个新文件， `time.rs`：

```rust
use crate::consts::*;
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

pub struct ControlledTime {
    delta: Duration,
    last_update: Option<Instant>,
    delta_seconds_f64: f64,
    delta_seconds: f32,
    seconds_since_startup: f64,
    startup: Instant,
}
impl Default for ControlledTime {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs(0),
            last_update: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            seconds_since_startup: 0.0,
            delta_seconds: 0.0,
        }
    }
}
```

Here we've added a struct that is the same as Bevy's time, with the same `Default` implementation, and we've called it `ControlledTime`.
>这里我们添加了一个与 Bevy 的 time 相同的结构体，使用相同的 `Default` 实现，我们将其称为 `ControlledTime`。

Now, we'll add the methods we'll be using, taken from [the source](https://github.com/bevyengine/bevy/blob/3b2c6ce49b3b9ea8bc5cb68f8d350a80ff928af6/crates/bevy_core/src/time/time.rs), but we'll also add a `reset_time` function, that set's the time to 0:
>现在，添加我们想要的方法，来自于[这个资源](https://github.com/bevyengine/bevy/blob/3b2c6ce49b3b9ea8bc5cb68f8d350a80ff928af6/crates/bevy_core/src/time/time.rs)，我们还会添加一个 `reset_time` 函数，它将时间设置为 0：

```rust
// time.rs
impl ControlledTime {
    pub fn reset_time(&mut self) {
        self.startup = Instant::now();
        self.seconds_since_startup = 0.0;
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.update_with_instant(now);
    }

    pub fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = instant - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.last_update = Some(instant);
    }

    /// The delta between the current and last tick as [`f32`] seconds
    /// 当前和最后一次标记的时间差是 [`f32`] 秒
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    /// 当前和最后一次标记的时间差是 [`f64`] 秒
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    /// 启动后的时间，以秒为单位
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }
}
```

With that out of the way, we'll need a system that updates the time:
>考虑到这一点，我们需要一个能够更新时间的“系统”：

```rust
// time.rs
pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}
```

And another system that resets it:
>并且有一个系统对时间进行重置

```rust
// time.rs
pub fn reset_time_when_entering_game(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}
```

We'll also add a Plugin to keep them all together:
>我们还会添加一个插件来把它们放在一起：

```rust
// time.rs
pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ControlledTime>()
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_time.system())
            .on_state_update(APP_STATE_STAGE, AppState::MakeMap, update_time.system())
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::Game,
                reset_time_when_entering_game.system(),
            )
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::MakeMap,
                reset_time_when_entering_game.system(),
            );
    }
}
```

We've set `update_time` to run during `Game` and `MapMaker`, and `reset_time_when_entering_game` to run on state enter for both of these states.
>我们在 `Game` 和 `MapMaker` 执行期间设置了 `update_time`，并且 `reset_time_when_entering_game` 在这两种状态下都会执行。

Like we've done with all other plugins, let's go into `main.rs` to add it:
>跟其它插件一样，我们在 `main.rs` 中添加：

```rust
// main.rs
mod time;
use time::TimePlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin) // <--- New
        .run();
}
```

Last thing we need is to replace some of the places where we use `Time` to instead use `ControlledTime`.
>我们需要做的最后一件事就是用 `ControlledTime` 代替 `Time`。

First one will be `ui.rs`, where we just need to change the `time` parameter in `update_time_text`:
>首先是 `ui.rs`，我们只需改变 `update_time_text` 中的 `time` 参数：

```rust
// ui.rs
use crate::time::ControlledTime;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    [...]
}
```

The same thing happens with `audio.rs`, where we'll just replace `Time` for `ControlledTime`:
>`audio.rs` 文件也一样，将 `Time` 替换为 `ControlledTime`

```rust
// audio.rs
use crate::time::ControlledTime;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    [...]
}
```

Last one is `arrows.rs`, where we do need to make a couple more changes:
>最后是 `arrows.rs` 文件，要修改的地方多一些：

```rust
// main.rs
use crate::time::ControlledTime;

/// Spawns arrows
fn spawn_arrows(
    commands: &mut Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<ControlledTime>,
) {
    [...]
}

/// Moves the arrows forward
fn move_arrows(time: Res<ControlledTime>, mut query: Query<(&mut Transform, &Arrow)>) {
    [...]
}
```

Run the game now to see the menu and game working correctly:
>现在运行游戏，可以看到菜单和游戏正常工作了：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4)

Awesome!
>太棒了！

## Adding a simple map maker mode
>添加简单的地图制作模式

In this section we'll add a mode to help us create maps for our songs. What we want is the song to play while we press the keys, and to collect those and save them in a file.
>在本节中，我们添加一个场景模式来帮助我们给歌曲创建地图。我们想要的是当我们按下按键时播放歌曲，并将它们收集到一个文件中。

Let's open a new file called `map_maker.rs`, where we'll start by adding a resource and a system:
>我们打开一个新文件 `map_maker.rs`，我们从添加资源和“系统”开始：

```rust
use crate::time::ControlledTime;
use crate::consts::*;
use crate::types::{
    ArrowTimeToml,
    Directions::{self, *},
    Speed,
};
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use serde_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Debug, Default)]
/// Keeps track of when keys are pressed
/// 跟踪按键被按下的时间
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

/// Saves key presses to Presses
/// 保存被按下的键
fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
) {
    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        if direction.key_just_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.seconds_since_startup(),
                speed: Speed::Slow,
                direction: *direction,
            });
        }
    }
}
```

A part from the ton of imports we have added, we've created the `Presses` resource, which will just keep a list of `ArrowTimeToml`, and a system that adds to that list when a direction's key is pressed, by looping through all directions.
>我们大量添加需要增加的东西，我们创建 `Presses` 资源，它保存了一个 `ArrowTimeToml` 列表，以及一个当方向键被按下时添加到该列表的“系统”，并循环所有方向的按键。

We're going to also need a system that listens to the `AppExit` event, and saves the `ArrowTimeToml` list into a file:
>我们还需要一个系统来监听 `AppExit` 事件，并将 `ArrowTimeToml` 列表保存到文件中：

```rust
// map_maker.rs
fn save_to_file_on_exit(
    mut event_reader: Local<EventReader<AppExit>>,
    events: Res<Events<AppExit>>,
    presses: Res<Presses>,
) {
    for _event in event_reader.iter(&events) {
        let text = toml::to_string(&*presses).expect("Couldn't convert to toml text");

        let mut file = File::create("map.toml").expect("Couldn't open map.toml");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml");
    }
}
```

We also want something to make this mode slightly more pleasant to use. We'll make it so that when the player is pressing a key, the corresponding direction shows on screen as an arrow. We'll add two systems, one to spawn the arrows and another to toggle visibility:
>我们得做点什么来提高这个模式得易用性。当玩家按下一个按键时，相应的方向会有箭头出现在屏幕上。我们将添加两个系统，一个生成箭头，一个切换箭头的可见性：

```rust
// map_maker.rs
struct MapMakerArrow(Directions);

/// Creates map maker arrows
fn setup_map_maker_arrows(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let border_handle = materials.add(asset_server.load("images/arrow_border.png").into());

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        let y = match direction {
            Up => 150.,
            Down => 50.,
            Left => -50.,
            Right => -150.,
        };

        let mut transform = Transform::from_translation(Vec3::new(0., y, 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                material: border_handle.clone(),
                sprite: Sprite::new(Vec2::new(140., 140.)),
                transform,
                ..Default::default()
            })
            .with(MapMakerArrow(*direction));
    }
}

/// Toggles visibility according to if corresponding key is being pressed
/// 根据是否按下对应的键来切换可见性
fn toggle_map_maker_arrows(
    mut query: Query<(&mut Visible, &MapMakerArrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut visible, arrow) in query.iter_mut() {
        visible.is_visible = arrow.0.key_pressed(&keyboard_input);
    }
}
```

The first system is very similar to `spawn_target_arrows`, it just creates the sprites, and adds the `MapMakerArrow` component we've just declared. The second system, `toggle_map_maker_arrows`, sets the visibility of those arrows according to if the arrow's direction is being pressed. We do this by setting the `is_visible` field in the `Visible` component of the sprite.
>第一个“系统”非常类似于 `spawn_target_arrows`，它只是创建精灵，并添加我们刚刚声明的 `MapMakerArrow` 组件。第二个系统是 `toggle_map_maker_arrows`，根据箭头对应的方向键是否被按下来设置箭头的可见性。我们通过设置精灵的 `Visible` 中的 `is_visible` 字段来做到这一点。

There's one issue, the `key_just_pressed` method we have currently declared for `Directions` uses `just_pressed`, which will only be true the first frame the key is being pressed. We want our arrows to show up for as long as the player has the key pressed, so we'll add another method that uses `pressed` instead, which does what we want:
>这里有一个问题，我们目前给 `Directions` 声明的 `key_just_pressed` 方法使用了 `just_pressed`，这只会在按键被按下的第一帧时才会生效。我们希望玩家按下按键，箭头就立即显示，所以我们添加了另一种 `pressed` 方法，它可以实现我们想要的：

```rust
// types.rs
impl Directions {
    [Other methods...]

    /// Checks if a key that corresponds to this direction is being pressed
    /// 检查是否按下与当前方向相同的方向键
    pub fn key_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.pressed(*code))
    }
}
```

With that, our `toggle_map_maker_arrows` system will work correctly! We'll also make a plugin for all of the map making things:
>这样，我们的 `toggle_map_maker_arrows` 系统就可以正常工作了！我们还要给所有的歌曲地图实现一个插件：

```rust
// map_maker.rs
pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::MakeMap,
                setup_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                toggle_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_key_presses.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_to_file_on_exit.system(),
            );
    }
}
```

The last thing we need to get this working is to add the system in `main.rs`:
>要想让它运行起来，我们还需要在 `main.rs` 中加上“系统”使用的调用：

```rust
// main.rs
mod map_maker;
use map_maker::MapMakerPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(MapMakerPlugin) // <--- 新增代码
        .run();
}
```

We can now run the game to see our map maker mode working correctly:
>现在，我们可以运行游戏来看看地图制作是否能正常工作：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_map_maker_mode.mp4)

Remember to exit using the ESC key, and not by `Ctrl+C` in the terminal, so that the file is saved successfully.
>请记住，在游戏终端中使用 ESC 键退出，而不是 `Ctrl+C` 键，这样才能保存文件成功。

And here is an example of the file that we'll get:
>这是我们得到的一个文件示例：

```map.toml
[[arrows]]
click_time = 1.04939044
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 1.658164574
speed = "Slow"
direction = "Down"

[[arrows]]
click_time = 2.191576505
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 2.558483463
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 2.858588189
speed = "Slow"
direction = "Left"

[[arrows]]
click_time = 3.4904190330000002
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 3.9252477949999998
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 4.240984206
speed = "Slow"
direction = "Left"

[[arrows]]
click_time = 4.62353972
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 4.97381796
speed = "Slow"
direction = "Up"

[[arrows]]
click_time = 5.308837329
speed = "Slow"
direction = "Left"
```

We could now add this to the `assets/songs` folder, adding the `name` and `filename` fields, and we'd have a working map for our song!
>现在我们可以将它添加到 `assets/songs` 目录下，添加 `name` 和 `filename` 字段，这样就有了歌曲的工作地图！

The last thing we need is to also play the song on the map maker mode, otherwise it's a bit useless. We're going to go the easy route, and we'll hardcode the path for the song used, this way this tutorial doesn't become unbearably long (if it hasn't already). We'll use the song at the path `assets/map_maker_song.mp3`. The player will have to change the file at that path to change the song used in the map maker. Feel free (and I actually encourage) everyone to make some kind of system to select the song used in the map maker more easily.
>我们需要做的最后一件事是在地图制作模式下播放歌曲，否则它就显得有点鸡肋。我们简单实现一下，并且给使用的歌曲路径硬编码，这样可以让教程简短一些（如果还算短的话）。我们将使用路径 `assets/map_maker_song.mp3` 中的歌曲。玩家必须在地图制作器中修改文件路径来更换歌曲。每个人都可以实现一些自己的“系统”，以更容易地选择地图制作器中使用的歌曲。

## 在地图制作器中播放歌曲
The first thing we'll do to get music going in map maker is to add a resource that holds the `Handle<AudioSource>`. We'll implement `FromResources` for this resource, so that we can load it at the start and it's ready to play as soon as we load into the map maker:
>为了让音乐进入地图制作器，我们先要添加一个资源来保存 `Handle<AudioSource>`。我们要为该资源实现 `FromResources`，这样可以在开始时就加载它，当把它加载到地图制作器中时，它就准备好可以玩了：

```rust
struct MapMakerAudio(Handle<AudioSource>);
impl FromResources for MapMakerAudio {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        let audio = asset_server.load("map_maker_song.mp3");
        Self(audio)
    }
}
```

This time I've decided to go with a tuple struct for the resource, as we only have one field. The `FromResources` implementation loads the asset server, and the uses it to load the audio.
>这一次我决定使用一个元组结构体来处理资源，因为我们只有一个字段。 `FromResources` 实现了静态资源服务器，它可以加载音频。

After that, we'll make a new system that starts playing the audio, and we'll set it to run on state enter for `MakeMap`:
>在那之后，我们要创建一个新“系统”来进行播放音频，我们将把它设置为进入 `MakeMap` 的状态时执行：

```rust
// map_maker.rs
fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}
```

The last thing we need is to add these two things to the plugin:
>我们要做的最后一件事是将这两个资源加到插件中：

```rust
// map_maker.rs
pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .init_resource::<MapMakerAudio>() // <--- New
            .on_state_enter(APP_STATE_STAGE, AppState::MakeMap, start_song.system()) // <--- New
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::MakeMap,
                setup_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                toggle_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_key_presses.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_to_file_on_exit.system(),
            );
    }
}
```

Get an audio file and put it into `assets/map_maker_song.mp3`, and if you run the game you should have audio playing when you start the map maker mode!
>找一个音频文件，并将其放到 `assets/map_maker_song.mp3` 中，如果你运行游戏，进入地图制作模式时，应该可以听到音频播放了！

With this, we've reached the end of our adventure. As always, feel free to experiment, change things up, and make it yours! If you make any changes, please tag me on [Twitter](https://twitter.com/guimcaballero) so I can see it!
>至此，我们的游戏教程就结束了。和往常一样，你可以随意尝试，修改一些东西，让它成为你的东西！如果你有任何的改进，请在 [Twitter](https://twitter.com/guimcaballero) 标记我，这样我就能看到了！

## 下一步
Here are some ideas you could try if you don't have a specific game of your own you want to work on:
>如果你还没想好要做什么样的二次开发，以下提供一些可以尝试的想法：

* 1.Add arrows that have to be held during a certain amount of time.
* 1.添加必须在特定的时间内保持状态的箭头。
* 2.Improve map maker, adding something to select the song.
* 2.改进地图制作器，增加选择歌曲的功能。
* 3.Add an end screen to the game.
* 3.给游戏增加一个游戏结束画面。
* 4.Add a way to go back to menu after song has finished.
* 4.增加一种歌曲播放完后，回到菜单的方式
* 5.Make a system that changes the threshold for clicking so that it's more relaxed if the player is having a hard time, and it's stricter if the player is doing good.
* 5.创建一个可以改变点击阈值的“系统”，可以让玩家在困难模式时选择简单模式，玩家很轻松则增加困难模式。
