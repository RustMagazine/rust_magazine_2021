# 「译」基于 Rust 用 Bevy 实现节奏大师游戏

>* Rhythm game in Rust using Bevy 译文（基于 Rust 用 Bevy 实现节奏大师游戏）
>* 原文链接：https://caballerocoll.com/blog/bevy-rhythm-game/
>* 原文作者：[Guillem Caballero Coll](https://github.com/guimcaballero)
>* 译文来自：[RustMagazine 2021 期刊](https://github.com/RustMagazine/rust_magazine_2021)
>* 译者：[suhanyujie](https://github.com/suhanyujie)
>* ps：水平有限，翻译不当之处，还请指正。
>* 标签：Rust, Bevy,  game, Rhythm game

>2021/2/8 - 77 min read

## 介绍
在这个教程中，我们基于 Rust 使用 Bevy 引擎实现一个节奏大师游戏。目的是展现如何用 Bevy 实现一些东西，特别是一些更高级的功能，如着色器，状态，和音频。

如果你想在进入学习之前看看最终的代码，你可以在[这里](https://github.com/guimcaballero/bevy_rhythm)找到仓库，并且下面是一个游戏视频：

[视频资源](https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4)

这款游戏很简单：箭头飞过屏幕，玩家必须在正确的时间内按下正确的方向键才能让箭头消失。如果玩家成功地做到了这一点，他们将获得积分。否则，箭头会旋转着掉下来。箭头会有不同的速度，每个箭头颜色不同。游戏还有一个选择歌曲的菜单，以及一个简单的地图制作器来帮助创建歌曲地图。

## Bevy
[Bevy](https://bevyengine.org/) 是一个数据驱动的游戏引擎。它使用起来非常简单，令人愉悦。它使用 [ECS](https://en.wikipedia.org/wiki/Entity_component_system) 来管理游戏实体及其行为。

Bevy 有一个很受欢迎的社区，所以如果你对本教程有任何疑问，可以查阅 [Bevy book](https://bevyengine.org/learn/book/introduction/)，浏览[示例]](https://github.com/bevyengine/bevy/tree/master/examples)，或者加入[官方的 Discord](https://discord.gg/gMUk5Ph) 进行提问。

如果你发现教程中存在错误，请在这里开一个 [Issue](https://github.com/guimcaballero/bevy_rhythm/issues)，我会修正它。

## 前期准备

在本教程中，你需要熟悉 Rust。你不必成为专家，我们不会使用任何的黑魔法。虽然不是必须的，但强烈建议你去了解一下 ECS 的工作原理。

如果你想阅读一些更简单的教程，我建议你阅读[基于 Rust，使用 Bevy 实现贪吃蛇](https://mbuffett.com/posts/bevy-snake-tutorial/)，或者[ Bevy 实现国际象棋](https://caballerocoll.com/blog/bevy-chess-tutorial/)教程，可以详细了解基础知识。

此外，我们将在本教程中使用着色器和 [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language)。这两种知识不是必须的，因为我会提供要使用的代码，但了解 GLSL 会使你可以修改更多的东西，并让游戏真正属于你自己的。

如果你之前从未使用过着色器，可以参考下面这些推荐链接开始学习：

* [Shadertoy 入门](https://www.youtube.com/watch?v=u5HAYVHsasc)：介绍并使用 [Shadertoy](https://www.shadertoy.com/)。
* Unity 着色器编码入门 —— [一款即兴的在线课程](https://www.youtube.com/watch?v=9WW5-0N1DsI)：介绍在 Unity 中使用着色器。非 Unity 官方指定的大部分资料都在这儿。
* [Unity 教程：着色器的实用介绍 —— 第一部分](https://www.youtube.com/watch?v=C0uJ4sZelio)：与上面类似。

## 创建一个项目
和往常一样，我们使用 `cargo new bevy_rhythm && cd bevy_rhythm` 创建一个空 Rust 项目。你现在可以打开该 crate 项目。并用你喜欢的编辑器打开 `Cargo.toml`，把 `bevy` 加入到依赖项中：

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
我建议你启用快速编译，以确保开发过程不会太烦躁。以下是我们需要准备的：

* 1.LLD 链接器：普通链接器会有点慢，所以我们把其换成 LLD 链接器进行加速：
    * Ubuntu: `sudo apt-get install lld`
    * Arch: `sudo pacman -S lld`
    * Windows: `cargo install -f cargo-binutils and rustup component add llvm-tools-preview`
    * MacOS: `brew install michaeleisel/zld/zld`
* 2.为该项目启用 Rust 的 nightly 版本：rustup 工具链安装 nightly 版，并且在项目目录中设置 rustup 为 nightly 进行启用。
* 3.把[这个文件](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds)的内容拷贝到 `bevy_rhythm/.cargo/config` 中。

以上就是所有要准备的事情了，现在运行游戏来编译所有的库。编译完成后，你应该在命令行中看到 `Hello, world!`。

注意：如果你看到游戏性能很差，或者看到[加载资源很慢](https://github.com/guimcaballero/bevy_rhythm/issues/1)，你可以用 `cargo run --release` 的编译模式下运行。编译时间可能会稍长一些，但游戏运行会更加流畅！ 

## 开始
任何 Bevy 游戏的第一步都是增加小段示例代码来启动应用的。打开 `main.rs`，并将已有的 `main` 函数替换为下面的内容：

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::build()
        // 抗锯齿设置 samples 为 4
        .add_resource(Msaa { samples: 4 })
        // 设置 WindowDescriptor 资源修改标题和窗口大小
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

如果你使用 `cargo run` 运行程序，你会看到一个空白窗口：

![](https://caballerocoll.com/images/bevy_empty_window.png)

这一步设置 Bevy `App`，添加默认插件。这将包括转换、输入、窗口等游戏运行所需的元素。如果你不需要这些功能， Bevy 是模块化的，你可以选择只开启你需要的功能。我们要新增这些插件，所以需要使用 `add_plugins` 和 `DefaultPlugins`。

我们还添加了两个资源：`Msaa` 和 `WindowDescriptor`，分别用于配置 anti-aliasing，以及窗口大小和标题。最后，我们添加了 Bevy 的 `exit_on_esc_system`，它的作用是按下 esc 键时关闭游戏。

## Bevy 中的 ECS
下面是 ECS 如何在 Bevy 中工作的介绍。如果你已经知道它是如何工作的，可以[跳过本节](https://caballerocoll.com/blog/bevy-rhythm-game/#adding-a-setup-system)。这和我们的游戏无关，我将使用 [Bevy book](https://bevyengine.org/learn/book/getting-started/ecs/) 中的例子来说明它是如何运作的。你不需要复制这里的代码，只需读懂它即可。

Bevy 的 ECS 是 [hecs](https://github.com/Ralith/hecs) 的一个分支版本。它使用 Rust 结构体作为组件，不需要添加宏或其他复杂的东西。例如：

```rust
// 有两个字段的结构体组件
struct Position { 
    x: f32,
    y: f32
}

// 元组组件
struct Name(String);

// 我们甚至可以使用标记组件
struct Person;
```

Systems are just normal Rust functions, that have access to `Querys`:
>这个“系统”中可以使用正常的 Rust 函数，访问 `Querys`：

```rust
fn set_names(mut query: Query<(&Position, &mut Name), With<Person>>) {
    for (pos, mut name) in query.iter_mut() {
        name.0 = format!("position: ({}, {})", pos.x, pos.y);
    }
}
```

一次查询可以访问组件中所有实体。在前面的示例中，`query` 参数允许我们迭代包括 `Person` 组件在内以及 `Position` 和 `Name` 等组件实体。因为我们用 `&mut Name` 替代 `&Name`，所以可以对实体进行修改。如果对 `&Name` 类型的该值进行修改，Rust 会报错。

有时候我们想要只在游戏开始时运行一次的机制。我们可以通过“启动系统”来做到这一点。“启动系统”和“普通系统”完全一样，唯一的区别是我们将如何把它加到游戏中，这会在后面进行详细讲解。下面是一个使用 `Commands` 生成一些实体的“启动系统”：

```rust
fn setup(commands: &mut Commands) {
    commands
        .spawn((Position { x: 1., y: 2. }, Name("Entity 1".to_string())))
        .spawn((Position { x: 3., y: 9. }, Name("Entity 2".to_string())));
}
```

Bevy 也有资源的概念，它可以保存全局数据。例如，内置的 `Time` 资源给我们提供游戏中的当前时间。为了在“系统”中使用这类资源，我们需要用到 `Res`：

```rust
fn change_position(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut pos in query.iter_mut() {
        pos.x = time.seconds_since_startup() as f32;
    }
}
```

我们自定义资源也很简单：

```rust
// 一个简单的资源
struct Scoreboard {
    score: usize,
}

// 另一个资源，它实现了 Default trait
#[derive(Default)]
struct OtherScore(f32);
```

我们有两种方法初始化资源：第一种是使用 `.add_resource` 并提供我们需要的结构体，另一种是实现了 `Default` 和 `FromResources` 的 `.init_resource`。

下面我们如何把它们加到游戏中：

```rust
fn main() {
    App::build()
        // 新增资源的第一种方法
        .add_resource(Scoreboard { score: 7 })
        // 第二种方法，通过 Default 的初始化加载资源
        .init_resource::<OtherScore>()

        // 增加“启动系统”，游戏启动时只会运行一次
        .add_startup_system(setup.system())
        // 增加一个“普通系统”，每一帧都会运行一次
        .add_system(set_names.system())
        .add_system(change_position.system())
        .run();
}
```

Another cool thing Bevy has are Plugins, which we've already seen when we used `DefaultPlugins` in the previous section. Plugins allow us to wrap features that belong together, which then let's us enable and disable them together easily. Plugins also provide organization, which is the main purpose we'll be creating our own in this tutorial.
>Bevy 还有一个很酷的东西是插件，我们在上一节使用 `DefaultPlugins` 时看到了。插件可以让我们将一些特性包装在一起，这可以让我们很容易地启用和禁用它，插件也提供了组织功能，这也是我们在这篇教程中自定义插件地主要功能点。

如果有些东西不清楚，不用担心，我们会在后面更详细地解释所有内容。

## 增加系统设置

每个游戏都需要一个相机来渲染对象，所以我们将从如何添加一个生成相机的“启动系统”开始。因为这是一款 2D 游戏，所以我们要使用 `Camera2dBundle`。

```rust
use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::build()
        // 设定[抗锯齿](https://cn.bing.com/search?q=%E7%BB%98%E5%88%B6+%E6%8A%97%E9%94%AF%E9%BD%BF&qs=n&form=QBRE&sp=-1&pq=%E7%BB%98%E5%88%B6+%E6%8A%97%E9%94%AF%E9%BD%BF)，samples 参数值为 4
        .add_resource(Msaa { samples: 4 })
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

bundle 是组件的集合。在本例中，`Camera2dBundle` 将创建一个包含 `Camera`，`OrthographicProjection`，`VisibleEntities`，`Transform` 和 `GlobalTransform` 的 实体。其中大部分是我们玩游戏时不需要用到的，所以我们使用抽象的 `Camera2dBundle` 添加组件。

注意：我们还可以使用一个元组代替 bundle 来添加所有组件：

```rust
fn setup(commands: &mut Commands) {
    commands.spawn((Camera::default(), OrthographicProjection::default(), VisibleEntities::default(), Transform::default(), GlobalTransform::default()));
}
```

这段代码实际上还不能运行，因为我们还需要在 camera 和投影组件中设置一些字段，但我觉得它明确地体现了使用 bundle 和元组来添加结构是很相似的。

## 加载精灵
在这部分中，我们会添加一些“精灵”，让它们四处移动。为此，我们需要创建一个 `assets` 目录，我们将存储一些[图像](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/images)和[字体文件](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/fonts)。目录中有两个子文件夹，图像和字体。你可以点击前面提到的链接，从 GitHub 仓库下载。

你的资源目录应该如下所示：

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

我们将使用带颜色的箭头来表示不同速度的箭头，并使用带边框的箭头来标记目标区域。

有了这些静态资源，我们就可以开始编写一些游戏动画了。我们将创建一个 `arrows.rs` 文件，它将包含生成，移动，清除箭头等相关操作。首先要做的是为“箭头精灵”保留资源，这样我们就不必在每次创建箭头时重新加载它们：

```rust
use bevy::prelude::*;

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

通过实现 `FromResources` trait，在我们调用 `.init_resource::<ArrowMaterialResource>()` 时，Bevy 会管理并初始化资源，在进程中加载图片。

如你所看到的，实际的资源加载是 `Handle<ColorMaterial>` 而不是 `ColorMaterials`。这样，当我们创建箭头实例时，我们可以使用对应的 handle，并且它们将复用已存在的资源，而不是每个都各自独有一份。

## 生成并移动箭头

我们接下来要做的是生成箭头并在屏幕上移动它们。我们从实现每秒生成一个箭头的“系统”开始。箭头会包含一个名为 `Arrow` 的空（结构体）组件：

```rust
/// 箭头组件
struct Arrow;

/// 跟踪何时生成新箭头
struct SpawnTimer(Timer);

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

在这个系统中，我们使用了 `Timer`，这是 Bevy 中执行每隔 `x` 秒重复操作的最佳方式。我们使用 [newtype 模式](https://rust-unofficial.github.io/patterns/patterns/newtype.html)进行封装，这样我们能够把 `SpawnTimer` 与其他的定时器区分开。我们需要使用形如 `.add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))` 的调用方式进行初始化，调用稍后会进行。将 `true` 作为参数值传递表示计时器结束时会再次重复执行。

要使用计时器，我们必须手动调用它的 `tick` 方法，入参 time 是距离上次调用所间隔的时间差，然后我们可以使用 `just_finished` 来查看定时器是否完成。实际上我们所做的是提前检查定时器是否完成来确保 `spawn_arrows` 系统每秒只运行一次。

系统的其余部分将创建一个 `Transform` 组件，我们将其添加到箭头组件中，它会返回 `SpriteBundle` 从而生成箭头，并给箭头实体一个来自 `ArrowMaterialResource` 的红色纹理。我们使用 `Commands` 中的 `with` 方法添加了 `Arrow` 组件。这样，我们创建的实体将拥有所有的 `SpriteBundle` 和 `Arrow` 组件。

注意：这个系统只是临时的，并且它会被在某个特定时间内生成箭头的东西所覆盖。

现在，我们生成的那些箭头就在那了，我们需要用另一个系统让它们向右移动：

```rust
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}
```

`move_arrows` 使用 `Query` 来获取所有带有 `Transform` 和 `Arrow` 组件的实体，并通过增加 x 坐标值来将它们向右移动一点点。我们还使用了 `Time::delta_seconds()` 来根据当前帧到上一帧的时间来增加距离。 

我们把这些 `ArrowMaterialResource` 和 `SpawnTimer` 等系统连接到一个插件中：

```rust
pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // 初始化资源
            .init_resource::<ArrowMaterialResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            // 增加 system
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}
```

我们现在可以将 `main.rs` 改为如下内容：

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

我们需要做的只是增加 `.add_plugin(ArrowsPlugin)`，这样所有的系统和资源就被正确地集成在 `arrows.rs` 中。

如果你运行程序，你会看到箭头在屏幕上飞舞：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_red_arrows_moving.mp4)

## 类型和常量

我们在上一节中对一些值硬编码了。因此我们需要重新使用它们，我们要新建一个小模块来保存我们的常量。创建一个名为 `consts.rs` 的文件，并添加以下内容：

```rust
/// 箭头移动的速度
pub const BASE_SPEED: f32 = 200.;

/// 箭头生成时的 X 坐标值，应该在屏幕之外
pub const SPAWN_POSITION: f32 = -400.;

/// 箭头应该被正确点击时的 X 坐标值
pub const TARGET_POSITION: f32 = 200.;

/// 点击箭头时的容错间隔
pub const THRESHOLD: f32 = 20.;

/// 箭头从刷出到目标区域的总距离
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;
```

其中一些常数稍后才会用到。在 `main.rs` 中增加 `mod consts`，以导入模块使其可用。我们可以在 `arrows.rs` 中的 `spawn_arrows` 和 `move_arrows` 替换掉对应硬编码的值。

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

/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * BASE_SPEED;
    }
}
```

现在我们的箭头在屏幕上移动，但他们都面向相同的方向、相同的速度移动，且颜色相同。为了能够区分它们，我们将创建两个不同的枚举，一个用于表示方向（上、下、左、右），一个表示速度（慢、中、快）。

注意：我们把它叫做 `Directions` 而非 `Direction`，因为后者是一个[ Bevy 枚举](https://docs.rs/bevy/0.4.0/bevy/prelude/enum.Direction.html)。通过给它取一个稍微不一样的名字，防止混淆带来的麻烦。

让我们创建一个 `types.rs` 文件，并把上面提到的枚举值放于其中：

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
    /// 检查相应的方向键是否被按下
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// 返回此方向的箭头的旋转角度
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// 返回此方向的箭头的 y 坐标值
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

首先，我们添加 `Directions` 枚举。并且已经实现了三种不同的方法。

`key_just_pressed`，用于检查被按下的方向键。我已经决定增加 `D, F, J, K` 作为可能的键，因为我键盘上的方向键比较小。如果你是 FPS 玩家，你可以使用 `W, S, A, D`，或者 VIM 世界的 `K, J, H, L` 来替代它们。

注意：如果你不太习惯使用迭代器，下面是用传统的方法实现 `key_just_pressed`：

```rust
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

`rotation` 表示我们需要将“箭头精灵”旋转多少度以将其指向正确的方向。`y` 表示箭头的 `y` 坐标值。我决定把箭头的顺序调整为 `Up, Down, Left, Right`，但如果你喜欢其他顺序，你可以自己修改。

```rust
#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// 返回箭头移动的实际速度
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// Speed 乘数
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}
```

接下来，我们添加了 `Speed` 枚举。我们实现了两个方法：一个是乘法，它表示箭头应该相对于 `BASE_SPEED` 所移动的距离；另一个是 `value`，它是执行乘法运算得到的值。

这是一部分代码，我不希望特别复杂！接下来要添加的类型是 `ArrowTime` 和 `SongConfig`。前者记录何时生成一个箭头，以及它的方向和速度。第二个将保存所有箭头实体的列表：

```rust
#[derive(Clone, Copy, Debug)]
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

我们的 `ArrowTime` 有个问题。在内部，我们需要知道箭头什么时候生成，但在生成它时，我们希望指定应该在什么时候点击它。因为每个箭头都有不同的速度，所以仅仅减去几秒是不够的。为了解决这个问题，我们要创建一个 `new` 函数，包含 `click_time`，`speed` 和 `direction`，并设置相应的 `spawn_time`：

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

为了进行测试，我们将创建一个函数，它返回硬编码的 `SongConfig`，其中包含了不同的速度和方向的箭头：

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

最后，我们可以进入 `main.rs` 并将 `setup` 系统修改成下方所示：

```rust
mod types;

fn setup(commands: &mut Commands) {
    let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(config);
}
```

注意：我们使用 `insert_resource` 替代 `add_resource` 或 `init_resource`，因为后者是 `AppBuilder`，前者是用在 `Commands` 中。

如果我们现在运行游戏，没有任何变化，但仍然是能运行的，这很棒！我们进入 `arrows.rs` 文件，修改它使它能根据 `SongConfig` 中的列表生成箭头。

## 定时生成箭头

现在我们有了一个要生成的箭头列表，我们可以删除所有定时器的内容，并修改 `spawn_arrows` 系统来检查每一帧刷出的箭头。

我们可以想到的第一个实现是循环遍历 `SongConfig` 中的所有箭头，并检查哪些箭头应该在当前帧中生成。这是可行的，但我们会在每一帧都循环遍历一个可能会很大的数组。我们硬编码的只有 5 个箭头，这不成问题，但一整首歌的情况下，箭头可能会超过 1000 个，就算电脑很快，玩家也不希望游戏让它们的 CPU “热”起来。

相反，我们将假设 `SongConfig` 中的箭头是有序的。我们需要在歌曲开始前将它们进行排序，这很简单。了解了这一点，我们只能先检查列表中的第一个箭头，如果它应该被生成出来，我们也会检查下一个箭头，一次类推，直到我们到达那个不需要再生成的箭头为止。由于箭头是有序的，如果一个箭头不需要生成，那么其后的箭头也无需生成。在这之后，我们需要移除列表中已经被生成的箭头。 

我们还需要给 `Arrow` 新增 `Speed` 和 `Directions` 字段：

```rust
// 在顶部
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
    // 我们得到了从启动到当前的时间（secs）以及到最后一次迭代的时间（secs_last），这样我们就可以检查是否有箭头应该在这个窗口中生成。

    // 歌曲在启动后 3 秒开始，所以减去 3 秒。
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // 计数器用于计算列表中产生和删除箭头数量
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // 列表是有序的，所以我们遍历检查直到第一个不满足条件为止
        // 检查箭头是否应该在当前帧和下一帧之间的时间点生成
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // 根据速度得到与之匹配的箭头素材（纹理）
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
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

    // 移除列表中生成的箭头
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}
```

上面这段代码，我们来分析一下它。

在“系统”开始时，我们先获取游戏已经开始多久了，以及“系统”最后一次运行的时间点。我们使用 [delta_seconds_f64](https://docs.rs/bevy/0.4.0/bevy/core/struct.Time.html#method.delta_seconds_f64) 来获取，它返回自最后一次游戏更新以来的时间。有了这两个值，我们就能知道该生成哪个箭头。因为 Bevy 不会每纳秒都更新（不代表所有的游戏引擎），所以如果只是简单地检查 `spawn_time` 是否等于当前时间会导致我们跳过需要处理的箭头。例如，我们可能有一个箭头，它刷出的时间被设为 `3.0`。Bevy 可以在 `2.99` 时运行这个“系统”，然后 `3.01` 时运行一次。由于箭头被指定为在 `3.0` 时生成，它就与运行“系统”的时间不匹配，导致它永远不会生成。

我们换个方法，在“系统”开始时检查当前时间和最后结束时的时间，对于上面的举例，在第二次运行该“系统”时，就会有 `secs = 3.01` 以及 `secs_last = 2.99`，因为我们的箭头产生的时间超过 `secs_last`，但小于下一帧的 `secs`，所以能够生成。大功告成！

有了这个，我们可以对 `move_arrows` 做一下小修改，让它兼顾速度的影响，可以使用我们之前创建的 `Speed::value()` 方法：

```rust
/// 把箭头向前移动
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();
    }
}
```

很酷，现在每个箭头都显示了正确的颜色，并以相应的速度移动：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_colored_arrows_moving_at_speed.mp4)

## 增加目标区域箭头

现在我们将使用 `border_texture` 去创造目标箭头，以便玩家能够知道何时应该按下按键。为此，我们将创建另一个“启动系统”，`setup_target_arrows` 以及一个标记组件，`TargetArrow`：

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

为了创建四个箭头，我们创建了一个有四个方向值的数组，然后循环调用 `border_texture` 和空的 `TargetArrow` 组件。

不要忘记在 `ArrowsPlugin` 中添加 `setup_target_arrows` 作为“启动系统”：


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

好了，我们现在把“目标区域箭头”准备好了。

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_target_arrows.mp4)

## 按键按下时清除箭头

现在我们有了目标箭头，我们接下来要实现一个“系统”，它的作用是，当箭头刷出时，并且如果在特定的阈值内，用户点击了正确的操作键，箭头就会消失。我们将创建一个名为 `despawn_arrows` 的新“系统”：

```rust
/// 用户在箭头到达尽头前按下正确的按键，箭头消失。
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // 检查按下按键时，是否是在特定的阈值内
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);
        }

        // 当箭头离开屏幕时，箭头消失
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
        }
    }
}
```

我们使用 `Query` 来查询所有实现了 `Transform` 和 `Arrow` 的实体。我们在查询中添加了 `Entity`，这样可以访问实体的“id”，然后我们可以在 `Commands::despawn()` 中根据它来消除实体。然后我们循环所有箭头，并检查 x 坐标值是否在点击的阈值内，如果是，则消除箭头。还有第二个检查，当箭头被错过离开屏幕时，它在最后也会被消除。它是在 x 坐标值大于等于 `2. * TARGET_POSITION` 时消除。 

记得用 `.add_system(despawn_arrows.system())` 将“系统”添加到 `ArrowsPlugin` 中，这样，运行游戏时，当我们斜着看的时候，也可以将其视为一种游戏！

## 增加基础 UI

在这一节中，我们将实现一些基本的 UI，目前只是显示了歌曲中的当前时间。我们会把它保存在 `ui.rs` 中：

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
        // 时间文本节点
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

在这个系统中，我们使用了父子关系模式（parenting），使得子实体可以相对于父实体进行转换。当我们把子实体加到父实体中后，给它一个合适的命名 `with_children`，它的参数是一个闭包，闭包接受一个类似于 `Commands` 的结构体类型 `ChildBuilder` 参数。在这个例子中，我创建了一个 `NodeBundle` 作为父实体，并将 `TextBundle` 作为子实体添加到其中。我们使用类似于 css 风格的 `Style` 组件让父节点坐落在屏幕的左上角。我们给文本实体增加了 `TimeText` 标记组件，这样我们就可以查询它，并且可以在任意帧中修改它。

现在，我们可以添加一个“系统”，它可以在每一帧中更新文本：

```rust
fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // 歌曲在实时启动 3 秒后开始
    let secs = time.seconds_since_startup() - 3.;

    // 在歌曲开始播放前不做任何处理
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.value = format!("Time: {:.2}", secs);
    }
}
```

该系统使用内置的 `Time` 资源，以及具有 `Text` 和 `TimeText` 的组件的实体查询。之后，我们只需要循环遍历它们并更新文本值。在实际情况中，应该只有一个实体能匹配上查询，所以我们可以只需获取第一个实体并完成此次操作，但无论如何我还是倾向于使用循环。这样，如果将来我们决定创建多个“系统”，我们就不必修改其中的代码了。

我们通过创建一个插件来完成该代码文件的编写：

```rust
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_time_text.system());
    }
}
```

现在，进入 `main.rs`，把 `CameraUiBundle` 加到 `setup` “系统”中，并导入插件：

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
        .add_plugin(UIPlugin) // <--- 新代码
        .run();
}

fn setup(commands: &mut Commands) {
    let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default()) // <--- 新代码
        .insert_resource(config);
}
```

`CameraUiBundle` 和 `Camera2dBundle` 很类似，但对于 UI 元素。如果不显式地添加它，文本就不会显示。因为我们之前已经添加了它，现在可以运行游戏，在屏幕上可以看到华丽地文字：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_time_text.mp4)

## 增加得分

在本节中，我们将创建得分系统，以便于玩家能过够在每次玩耍后看到自己的表现。为此，我们打开另一个文件 `score.rs`。在其中，我们将创建一个新的资源来记录分数以及正确的箭头和失败的箭头数量：

```rust
use crate::consts::*;

#[derive(Default)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,
}

impl ScoreResource {
    /// 增加合适的次数值以及得分
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // 根据按下的按键的及时性获取一个 0 到 1 的值
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // 最少增加 10 分，最多不超过 100 分。
        let points = (score_multiplier * 100.).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

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

`ScoreResource` 是一个简单的结构体，它有三个 `usize` 类型的私有字段。我们没有将字段设计成公有，而是设计成成员属性的 getter 和 setter。通过这种方式，增加合适的箭头数量的唯一方法是通过 `increase_correct`，它也能增加积分，我们需要保证有了这个方法后不会又编写另一个类似功能的方法。在这款游戏中，我们不需要这样，因为我们只需在一个地方增加分数，但对于其他更大的项目而言，这种做法更让我们有信心维护，它不会造成意料之外的漏洞。

我们把这个资源添加到 `main.rs`，并加上下面的引入代码：

```rust
mod score;
use score::ScoreResource;
```

使用下面的代码替换 `main` 函数：

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

完成之后，我们就能使用“系统”上的资源了。也就是说，我们对 `arrows.rs` 文件中的 `despawn_arrows` 系统做一些调整，这样，当箭头消失时，就会触发调用增加积分方法：

```rust
use crate::ScoreResource;

/// 当它们到达终点时，正确点击了按钮，就会消除箭头
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    
    // 新代码
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // 检查箭头是否是在阈值内点击的
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);

            // 新代码
            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // 离开屏幕时，箭头消失
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);

            // 新代码
            score.increase_fails();
        }
    }
}
```

改动很简单，我们增加 `mut score: ResMut<ScoreResource>` 作为系统的参数，以便我们可以编辑得分，我们添加了一个 `increase_correct` 方法，它会帮助我们增加积分，并且还有一个 `increase_fails` 方法，用于表示箭头离开屏幕消失时，积分增加失败。

现在，拥有一个得分系统很不错，但如果玩家无法看到自己的表现，那就没啥价值了！我们需要在 UI 模板中加一些东西，以显示分数：

```rust
use crate::ScoreResource;

// 新代码
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
            .add_system(update_score_text.system()); // <--- 新代码
    }
}
```

在 `update_score_text` 中，我们使用 `ChangedRes`，而非普通的 `Res`。它们的区别在于后者会在每一帧都会运行一次，而 `ChangedRes` 只会在资源发生改变时才会运行。这很酷，因为分数不会再每一帧里都发生变化，所以这样可以节省一些开销，只需在需要时才更新文本。然后，它在具有 `ScoreText` 组件的实体上设置文本值（和 `TimeText` 一样，应该只有一个，但为什么要限制）。

我们还要修改 `setup_ui` 中的一些东西，在第二次产生 `NodeBundle` 和 `TextBundle` 时，使用 `ScoreText` 组件：

```rust
fn setup_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time 文本节点
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
        
        // 新代码
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

我已经打算把这个文本设置在屏幕的左下角，但如果你想练习，你可以尝试把它设置在左上角时间文本的下面。

试试吧！运行游戏，看看我们的成果如何：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_score_text.mp4)

你可以随心所欲地为 UI 增减东西！我们在这里所做的是比较基础地展示文本。

## 从配置文件中加载数据

目前我们游戏中的箭头是硬编码的。目前这一切都还好，但我们希望玩家能创作自己的歌曲。我们不会通过制作自定义文件格式或任何花哨的东西使配置复杂化，所以我们将通过 [TOML](https://en.wikipedia.org/wiki/TOML) 和 [serde](https://github.com/serde-rs/serde) 库，来使用经过试用和测试的 [TOML](https://en.wikipedia.org/wiki/TOML) 格式。这两个 crate 将帮助我们非常容易地实现 `SongConfig` 结构的 TOML 序列化和反序列化。

向 `Cargo.toml` 文件加入以下内容：

```toml
toml = "0.5.8"
serde = "1.0.118"
serde_derive = "1.0.118"
```

我们现在可以编辑 `types.rs` 文件，并且导入准备好的类型和反序列化格式，向 `Directions` 和 `Speed` 类型中增加 `Deserialize` 和 `Serialize` trait 实现声明：

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

现在，我们有个小问题。我们的 `ArrowTime` 结构体有 `spawn_time` 字段，但是我们想在 TOML 文件中写入点击时间，所以我们不能直接在 Serde 中使用 `ArrowTime` 和 `SongConfig`。我们会通过创建两个新结构体来解决这个问题，`ArrowTimeToml` 和 `SongConfigToml`，它们对应的数据将会被包含在 TOML 文件中：

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

`name` 字段用于存储歌曲的名称，`filename` 是音频文件的路径，`arrows` 是 `ArrowTimeTomls` 列表。`ArrowTimeToml` 和 `ArrowTime` 的字段大部分一样，不同的是前者有 `click_time`，后者没有，取而代之的是 `spawn_time`。

我们也会把 `ArrowTime::new` 的入参改为 `ArrowTimeToml` 类型：

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

让我们在 `SongConfig` 加几个字段，用来保存名称和音频：

```rust
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}
```

我们用 `Handle<AudioSource>` 保存音频，当我们把 `SongConfigToml` 转换为 `SongConfig` 时，我们会使用 `AssetServer` 加载它。

最后，我们将修改 `load_config` 来从文件中加载 `SongConfig`：

```rust
pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // 打开文件并读取内容
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    // 使用 toml 和 Serde 进行解析
    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

    // 处理箭头
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<ArrowTime>>();
    // 根据 spawn_time 对箭头排序
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // 加载音频歌曲，并进行处理
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}
```

只有几行代码，但是很直接：先打开文件并读取文件的内容，使用 toml 库中的 `from_str` 方法解析文件内容，然后修改 `ArrowTimeTomls` 数组为 `ArrowTimes` 数组，我们使用 `AssetServer::load` 加载歌曲音频，然后返回新构建的 `SongConfig`。

注意：`AssetServer::load` 将在 `assets` 文件夹中搜索文件。`File::open` 不会从根目录开始查找，所以我们需要手动地将 `assets` 加到路径前缀中。

我们还需要修改 `main.rs` 中的 `setup` “系统”，修改 `load_config` 的调用方式，把 `AssetServer` 作为参数：

```rust
fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(config);
}
```

我们将在 `assets` 中创建一个 `songs` 文件夹，可以在其中保存所有的歌曲文件和对应的音频。现在，我们将创建一个名为 `test.toml` 的占位文件。你可以随意修改 arrows 以获得更详细的内容，现在只做一些简单测试：

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

现在，（合法地）下载你最喜欢的歌曲，将其放在 `assets/songs` 中，并将其命名为 `audio.mp3`。

你的 assets 目录应该如下方所示：

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

现在运行游戏，应该和上一节没有太大不同，只是你得到的箭头是根据外部文件配置加载的！如果你问我的话，我觉得相当酷 :)。

## 播放音频

你可能注意到，在上一节中，我们做了一些加载歌曲的逻辑，但当我们玩游戏时，歌曲还是不能播放。现在，我们来实现播放！为此，我新建了一个文件，`audio.rs`，其中只含有一个“系统”：

```rust
audio.rs
use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, config: Res<SongConfig>) {
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

`start_song` 使用 `Audio` 资源，在进入游戏 3 秒后开始播放歌曲。如你所看到的，我们使用了与“生成箭头”相同的方法。

注意：我们本可以复用 `Timer`，但当我们制作一个菜单来选择歌曲时，会带来一定的复杂度。何况尝试使用定时器重写，是个很不错的练习方式！

在 `main.rs` 中，我们添加以下内容：

```rust
// main.rs
mod audio;
use audio::AudioPlugin;
```

在 `main` 函数中，在所有插件加载的最后，添加 `.add_plugin(AudioPlugin)`。现在运行游戏应该会让歌曲播放了，因为计时器在运行！

至此，我们完成了游戏核心实现。你可以自由地在此基础上构建你自己地东西，但我建议你再往后看看，因为我们将致力于让游戏更加✨漂亮✨。

## 美化失败的箭头
首先，我们可以改进失败箭头的外观。目前，它们只是飞向远处。我们希望给玩家一些暗示，提醒他们那个箭头失败了。

我们要做的是让箭头在穿过目标区域后，“脱离”那条线。为了实现这一点，我们在 `arrows.rs` 中的 `move_arrows` 函数中加点东西：

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

我们所做的是获取目标到目标区域箭头符号的 `x` 坐标距离差，如果是正的，意味着它已经移动到目标区域外，我们就在它的 `y` 坐标减去一点，这样它就会下降。通过 `time.delta_seconds() * distance_after_target`，我们让每一帧的下降因子变大，这会让箭头以弧线的形式下降。`2.` 只是一个特定的常量，使弧线更好看（我觉得是），你可以根据你自己的意愿调整它！

效果见下方链接的视频：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_arrows_falling.mp4)

很好，我们再给它加点效果。我们让箭头在下降时收缩并旋转：

```rust
/// 箭头前移
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // 一旦箭头穿过目标区域，则开始下落
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;

            // 根据箭头地距离改变下降因子（比例）
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            // 根据距离和速度旋转箭头
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}
```

这是一串充满魔力的数字和公式，我在经过多次不同的尝试得出的结论。我建议你试试其它内容！

我们将其分析一下：首先，我们使用一个随着箭头移动而减小的公式来获得一个比例。然后，使用 `max` 来确保比例至少为 `0.2`。之后，我们使用 [Transform::rotate](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.Transform.html#method.rotate) 来旋转箭头。对于旋转，我们使用 `Speed::multiplier`，如果箭头的速度更快，就会旋转地更快。下面是所有这些效果组合在一起的样子：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_arrows_spinning.mp4)

太酷了！再次强调，你可以随时即兴发挥，添加其他逻辑，让它更加酷炫。游戏有一半的乐趣来自于制作你喜欢的花哨特效！

## 着色器背景

接下来我们要做的是替换灰色背景。选择之一是使用 `ClearColor` 资源，以静态颜色作为背景。[这里](https://github.com/bevyengine/bevy/blob/v0.4.0/examples/window/clear_color.rs)是一个使用示例。这种方式很简单，我们只需要在 `main` 函数中加上 `.add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))`，缺点是只能将背景改为一个平面颜色，我们希望看到更加生动的内容。着色器可以帮助我们！

我们将在所有元素下面制作一个窗口大小的精灵，我们将添加着色器材料。这样我们会有一个背景，也就是设置一个着色器作为背景。

当我们用着色器添加一些其他东西时，我们创建一个名为 `shaders` 的文件夹，用于存放相关文件。我们先打开 `shaders/mod.rs`：

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

现在，我们只添加了一些导入，声明了 `background` 模块，接下来就创建这个模块：

```rust
use super::*;

pub struct Background;
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
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

在这个文件中，我们添加了一个“启动系统”，它首先创建了 `PipelineDescriptor`，其中包含顶点和 fragment 着色器。这些都是用 `include_str` 宏从文件中添加进来的。然后我们会创建一个带有 `RenderPipelines` 组件的 `SpriteBundle`，并将我们创建的管道描述符传入。最后，我们添加了一个 `Background` 标记组件。

我们正在使用 `WindowDescriptor` 资源来得到屏幕宽度和高度，这样就可以进行正确的转换。如果玩家将窗口变大，会出现一个小问题，因为我们的背景大小不变，导致后面的灰色背景被显示出来！为了解决这个问题，我们添加另一个“系统”：

```rust
/// 当窗口大小变化时，背景大小跟着改变
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

它监听 [WindowResized](https://docs.rs/bevy/0.4.0/bevy/window/struct.WindowResized.html) 事件，该事件在每次调整窗口大小时会提供新的窗口宽高。

正如你注意到的，在 Bevy 中有一种易于使用且优雅的模式。事件也不例外。要使用一个事件，我们需要添加一个 `Event<T>` 资源和一个 `Local<EventReader<T>>` 作为参数。然后我们就可以通过事件资源来使用 `EventReader::iter`，该事件资源将给我们提供需要处理的事件。

实际使用着色器时是使用 Rust 的 `include_str` 宏添加的，它将以字符串的形式添加文件内容。首先，我们创建 `background.vert`：

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

我们在这里只需做一件特殊的事是添加 `v_Uv`（纹理的 uv 坐标）作为输出，这样，我们就可以在 fragment 着色器中使用它，现在我们在 `background.frag` 中创建它：

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

在这个着色器中，我们只返回基于背景的 uv 坐标的简单颜色。

我们现在需要注册这些创建的“系统”。我们在 `shaders/mod.rs` 中添加 `ShaderPlugin`：

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

现在我们可以在 `main.rs` 中导入它：

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

运行游戏你可以看到下方链接视频中展示的效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_simple_background.mp4)

## 使用时间着色器

继续，我们会有一些奇特的场景，酷！理想情况下，我们希望游戏背景随着时间有一些变化。

Bevy 没有（至少现在没有）添加时间和分辨率到着色器中作为输入，所以我们将不得不手动添加它们。希望这点能在 Bevy 中尽快得到改善。

我们再次打开 `shaders/mod.rs`文件，并增加以下代码：

```rust
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
/// 将资源传递给着色器
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

/// 在每一帧中，更新 ShaderInputs 中的时间
fn update_time(time: Res<Time>, mut nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    for mut node in nodes.iter_mut() {
        node.time = time as f32;
    }
}

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

/// 在渲染图形时，添加 ShaderInputs 作为一个 edge
fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}
```

我们正在创建一个新的 `ShaderInputs` 结构体，将其作为渲染图形边添加到 `setup_render_graph` 中，并将其加到“启动系统”中。`update_time` 和 `update_resolution` 是两个负责更新 `ShaderInputs` 组件值的系统。注意在 `update_resolution` 中我们是通过监听 `WindowResized` 事件来实现，而非更新每一帧。

现在，用以下代码替换 `ShaderPlugin` 中的实现，添加所有系统和资源：

```rust
pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ShaderInputs>() // <--- 新代码
            .add_startup_system(setup_render_graph.system()) // <--- 新代码
            .add_system(update_time.system()) // <--- 新代码
            .add_system(update_resolution.system()) // <--- 新代码
            .add_startup_system(setup_background.system())
            .add_system(update_background_size.system());
    }
}
```

我们现在要向之前创建的背景实体添加 `ShaderInputs` 组件，提供初始值：

```rust
// shaders/background.rs
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
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

这些值在添加一些东西后，现在可以在着色器上使用了：

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

基本上，我们必须对 `ShaderInputs` 的每个字段增加 uniform，它包含 `binding` 对应增加的值，以及形如 `ShaderInputs_$name` 的名字，其中的 `$name` 是字段名。现在我们可以使用着色器内部的变量了！

现在看起来应该如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_background_with_time.mp4)

就个人而言，我选择了以下配置的着色器作为背景：

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

它移动周围的颜色，产生好看的波浪，效果如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_fancy_background.mp4)

现在轮到你玩它了，找到你喜欢的东西。如果你不太理解着色器，你可以尝试对上面的着色器做一些小修改，你也可以去 [Shadertoy](https://www.shadertoy.com/) 查找一些资料。例如，下面是一个 [shader](https://www.shadertoy.com/view/XsXXDn) 配置，它由 Danilo Guanabara 转换自 Shadertoy：

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

效果如下方链接视频所示：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_creation_background.mp4)

## 美化点击动画

我们之前已经为失败的箭头添加了有趣动画，但当成功命中箭头时，我们啥也没做。它就这样消失了，这有点让人失望。我们将这一点进行改进。

我们将有四个不同的“精灵”，每个精灵在每个目标区域箭头下都有一个着色器。然后，每当正确命中箭头时，相应的精灵下的着色器就会启动动画，动画持续一段时间后，再消失。

注意：这个如果用技术实现会比较复杂，但这样可以展示很多东西。实现这一点有个捷径是在每次正确点击箭头时创建一个精灵，然后几秒钟后删除掉。

打开 `shaders/target_arrows.rs` 文件。我们为这些精灵添加一个组件（我把它叫做“普通目标箭头”），它只是指示目标箭头的方向和位置：

```rust
pub struct TargetArrowSparkle {
    direction: Directions,
}
```

我们再添加另一条边到渲染图中，并将另一个结构体作为参数传递给着色器。这将保留最近一次正确命中箭头的时间，以及对应得分：

```rust
// shaders/target_arrows.rs
#[derive(RenderResources, TypeUuid)]
#[uuid = "c9400817-b3a3-4baa-8bfa-0320b9b87b17"]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}
```

请注意，当我们向目标箭头添加 `TimeSinceLastCorrect` 组件时，每个组件都有自己的值，这些值是不共享的，所以我们需要单独设定它们。

现在，我们添加一个“启动系统”用于创建精灵：

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

这个系统就像是 `setup_target_arrows`，`setup_render_graph` 和 `setup_background` 的混合体。我们首先创建一个 `PipelineDescriptor`，然后添加 `TimeSinceLastCorrect` 作为渲染图的边，最后我们创建一个存放所有方向的数组，然后迭代它，创建 4 个精灵组，并添加 `TargetArrowSparkle`，`TimeSinceLastCorrect` 和 `ShaderInputs` 组件。

我们把 `last_time` 设为 3 秒进行测试。这样，当时间达到三秒时，动画就开始了。当我们设置好所有内容后，我们会将其更改为负值，因为我们希望箭头在被正确点击时触发。

我们还需要为这个着色器创建新文件：

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

vertex 着色器的实现基本上和 `shaders/background.vert` 一样。更有趣的是 `shaders/target_arrows.frag`：

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

这个着色器有点复杂，但简而言之，它的作用是创建一个半径随时间增加的圆。圆圈在 `last_time` 后存在 `0.6` 秒。我们把值设为 3 来添加 `TimeSinceLastCorrect`，并且和 `ShaderInputs` 一样，每个字段的绑定值都会增加。圆形的颜色根据点的不同而有所变化。

我们还需要把 `setup_target_arrows` 加到 `ShaderPlugin` 中：

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

现在运行游戏，将看到如下面链接视频所展示的效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_half_done_fancy_clicking.mp4)

如你所看到的，就在歌曲开始后，第 3 秒时，所有的圆圈开始变大，约过半秒后它们就消失了。太好了，这意味这着色器和定时器都正常工作了！我们仍然缺少一些东西来更新一些值，所以我们添加一个“系统”，用于当箭头被正确的按下时，更新 `last_time` 值。在此之前，我们使其默认值为负的：

```rust
// shaders/target_arrows.rs
.with(TimeSinceLastCorrect {
    last_time: -10.,
    points: 0.,
})
```

现在如果你运行这个游戏，圆圈就不会出现了。

之前，我们已经看到了如何侦听事件，但我们仍然没有看到硬币的另一面。我们现在就准备探索一下。我们将创建一个正确点击箭头时发生的事件。我们在 `arrows.rs` 文件中的 `despawn_arrows` 中产生这个事件：

```rust
// arrows.rs
/// 事件结构体
pub struct CorrectArrowEvent {
    pub direction: Directions,
    pub points: usize,
}

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

            // 新代码
            
            // 发送事件
            correct_arrow_events.send(CorrectArrowEvent {
                direction: arrow.direction,
                points,
            });
        }

        // 当箭头离开屏幕时消除它们
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
            score.increase_fails();
        }
    }
}
```

我们首先要做的是创建一个新的 `CorrectArrowEvent` 结构体，它用来表示我们的事件。对于 `despawn_arrows`，我们添加了 `ResMut<Events<CorrectArrowEvent>>` 参数，这样我们就能通过 `send` 方法发送事件。为了发送一个事件，我们需要传入一个 `CorrectArrowEvent` 结构体，它携带箭头的方向以及玩家的得分。

现在我们需要把 `.init_resource::<Events<CorrectArrowEvent>>()` 添加到 `ArrowsPlugin`，我们已经准备好了。很简单，对吧？

现在我们要在 `shaders/target_arrows.rs` 中添加一个“系统”，它负责更新“目标区域箭头”中的 `last_time`：

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

它通过监听事件，寻找与目标方向相关的箭头精灵，并更新其中的 `last_time` 和 `points` 值。

把最后一个“系统”加到 `ShaderPlugin`，`.add_system(correct_arrow_event_listener.system())`。现在如果你运行游戏，当你正确点击箭头时，就会看到圆圈效果：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_target_arrow_circles.mp4)

这就是这个游戏中我们要做的所有着色工作。和以往一样，你可以随便修改代码，添加更多效果，进行实验！

## 增加状态

在下一节，我们将制作一个非常简单的歌曲选择菜单。为此，我们将在一些状态值上下手，这就需要修改一些地方。为了创建一个状态，我们需要新建一个新的枚举，并将其包装成 [State](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.State.html) 的资源加到游戏代码中。然后，我们可以使用 `on_state_update`，`on_state_enter` 和 `on_state_exit` 等方法为每个系统分配特定的状态。

我们开始吧。首先，打开 `consts.rs`，添加 state 枚举：

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

`AppState` 将代表我们游戏的三个模式：歌曲选择菜单，游戏和（尚未实现的）地图制作模式。

我们，还添加了一个字符串用于表示我们的系统的阶段。现在我们进入 `main.rs` 中，添加 `State` 以及更新后的新阶段两个资源：

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
        .add_resource(State::new(AppState::Menu)) // <--- 新代码
        .add_stage_after( // <--- 新代码
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

现在游戏不会有任何变化，因为我们的“系统”仍然以普通的方式加入。为了改变这一点，我们将从修改 `arrows.rs` 中的 `ArrowsPlugin` 入手：

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

我们必须把 `add_startup_system`替换为 `on_stage_enter`，将 `add_system` 替换为 `on_stage_update`。对于这些函数，我们必须传入“系统”运行的阶段和状态。因为我们想要所有这些运行在 `Game` 状态，就是我们使用的那个。

现在我们看看 `ui.rs`：

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

`audio.rs` 中的代码:

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

我们已经修改了所有与 `Game` 状态相关的“系统”，所以如果你现在运行游戏，除了看到动画背景外，什么也不会发生，因为我们要从 `Menu` 开始，但是我们还没有相关的“系统”。

## 添加基础菜单

我们现在将制作一个带有按钮的菜单，它可以让我们选择一首歌曲或进入游戏地图制作模式。我们将它保存在一个新的文件 `menu.rs` 中。我们新建一个资源来保存对应的素材：

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

这看起来很标准。接下来，我们将创建一个“系统”来构建菜单元素。

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
            // 生成新按钮
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

这看起来非常类似于 `ui.rs` 中的 `setup_ui`。但结构类似于 `NodeBundle > ButtonBundle > TextBundle`。

我们还要创建一个删除所有按钮的系统，这样我们就可以在离开菜单时运行它。如果不这样做，菜单按钮会一直停留在游戏屏幕上。

```rust
// menu.rs
fn despawn_menu(commands: &mut Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}
```

给这个系统实现插件：

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

把它添加到 `main.rs` 中，导入它并在 `main` 函数中增加 `.add_plugin(MenuPlugin)` 调用：

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
        .add_plugin(MenuPlugin) // <--- 新代码
        .run();
}


fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
```

我们还要更改 `setup`，不再是 `SongConfig` 资源，因为我们会在玩家点击按钮选择歌曲时添加它。

现在运行游戏会显示下面这样的按钮：

![](https://caballerocoll.com/images/rhythm_basic_menu.png)

目前，单击按钮并将鼠标悬停在按钮上会发现按钮什么也没有干，所以我们需要让菜单能根据需要有所反应。首先，我们将添加一个系统，根据按钮的交互改变颜色：

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

这里我们使用的是 `Interaction` 组件，它和 `ButtonBundle` 一起。它有三个不同的变体，`Clicked`，`Hovered` 和 `None`。分别表示：单机按钮，悬停在按钮上，不做任何事。我们将匹配按钮的所有可能的值，从而做出不同的反应。将 `MenuPlugin` 加到游戏中，运行游戏，观察鼠标悬停、点击或移开时按钮的颜色是如何变化的。

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_button_interactions.mp4)

## 优化菜单

我们还需要两个东西：在文件夹中显示歌曲列表菜单，以及正式开始游戏的按钮。我们从第一点开始，在 `menu.rs` 中增加一个方法：

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

这个函数使用 [`read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) 获取 `songs` 目录中的文件，并将 `.toml` 后缀文件路径追加到数组中。

现在我们可以从 `setup_menu` 内部调用这个函数，来为 `get_songs` 得到的每个文件增加按钮。首先，我们创建一个枚举组件加到按钮中：

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

枚举的第一个变体 `MakeMap` 用于进入地图制作模式（如果实现了）。另一个变体 `PlaySong` 用于开始特定的歌曲游戏。

```rust
// menu.rs
fn setup_menu(commands: &mut Commands, button_materials: Res<ButtonMaterials>) {
    // 制作按钮列表
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
            // 将所有按钮以子按钮的方式加入
            for button in buttons {
                // 生成新按钮
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

我们已替换了 `with_children` 的内容，来循环遍历按钮列表，从而创建按钮。

注意：我们设置按钮的方式有点菜，所以如果你有很多按钮显示的话，它会看起来很奇怪！添加一个滚动条或者其他改善方式就留给读者作为练习了。

效果如下图所示：

![](https://caballerocoll.com/images/rhythm_menu_with_correct_buttons.png)

现在我们要让按钮可用。为此，我们添加另一个“系统”来监听点击事件：

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
        // 在这一帧中检测按钮是否被点击
        if *interaction == Interaction::Clicked {
            match button {
                // 如果地图制作按钮被点击，改变模式
                MenuButton::MakeMap => state
                    .set_next(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                // 如果它是一个播放歌曲按钮，加载对应配置，插入资源，然后改变态模式
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

在这个系统中，我们循环遍历每个按钮，并检查它们是否处于点击状态。如果是，我们会匹配按钮的类型，执行相应的逻辑。对于 `MakeMap`，我们只需使用 `set_next` 改变状态。对于 `PlaySong`，用我们创建的 `SongConfig` 函数来加载选定歌曲的 `SongConfig`，在将状态更改为 `Game` 之前，我们使用 `insert_resource` 添加歌曲。

最后，我们应该把这个系统添加到 `MenuPlugin`，设置成 `Menu` 状态更新时运行：

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

现在运行游戏，我们会看到按钮正常工作，开始游戏：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_menu_changing_state.mp4)

但有个大问题！当我们开始游戏时，时间在跑了，箭头却没有显示！因为我们使用 `time_since_startup` 来检查何时生成箭头，当我们进入 `Game` 状态时，值已经过了第一个箭头的生成时间，所以不会出现，其它箭头也不会出现。为了解决这个问题，我们将在后面制作一个包装器，这样我们就可以在进入 `Game` 模式时重置它。

## 时间系统封装

我们的时间包装器非常类似于 Bevy 的时间资源实现，不同的是它需要在我们进入 `Game` 和 `MakeMap` 状态时重置时间系统。复制所有代码只是为了改善一些糟糕的东西，但这会让我们在未来做其他工作时带来方便，比如暂停。这也是一个了解 Bevy 源码的好机会。

此外，通过同时拥有一个正常的时间资源和我们自己包装的版本，可以让我们使用正常的时间资源，以及其他需要控制时间的场景。例如，我们要继续为游戏背景使用正常时间，因为我们希望它在所有状态下都能工作。

打开一个新文件， `time.rs`：

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

这里我们添加了一个与 Bevy 的 time 相同的结构体，使用相同的 `Default` 实现，我们将其称为 `ControlledTime`。

现在，添加我们想要的方法，它来自于[这个资源](https://github.com/bevyengine/bevy/blob/3b2c6ce49b3b9ea8bc5cb68f8d350a80ff928af6/crates/bevy_core/src/time/time.rs)，此外我们还会添加一个 `reset_time` 函数，它将时间设置为 0：

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

    /// 当前标记和最后一次标记的时间差是 [`f32`] 秒
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// 当前标记和最后一次标记的时间差是 [`f64`] 秒
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// 启动后的时间，以秒为单位
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }
}
```

考虑到这一点，我们需要一个能够更新时间的“系统”：

```rust
// time.rs
pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}
```

并且有一个系统对时间进行重置

```rust
// time.rs
pub fn reset_time_when_entering_game(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}
```

我们还会添加一个插件来把它们放在一起：

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

我们在 `Game` 和 `MapMaker` 执行期间设置了 `update_time`，并且 `reset_time_when_entering_game` 在这两种模式下都会执行。

跟其它插件一样，我们在 `main.rs` 中添加：

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

我们需要做的最后一件事就是用 `ControlledTime` 代替 `Time`。

首先是 `ui.rs`，我们只需改变 `update_time_text` 中的 `time` 参数：

```rust
// ui.rs
use crate::time::ControlledTime;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    [...]
}
```

`audio.rs` 文件也一样，将 `Time` 替换为 `ControlledTime`

```rust
// audio.rs
use crate::time::ControlledTime;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    [...]
}
```

最后是 `arrows.rs` 文件，要修改的地方多一些：

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

现在运行游戏，可以看到菜单和游戏正常工作了：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4)

太棒了！

## 添加简单的地图制作模式

在本节中，我们添加一个场景模式来帮助我们给歌曲创建地图。我们想要的是当歌曲播放时，我们何时按下按键，并将它们保存到一个文件中。

我们打开一个新文件 `map_maker.rs`，我们从添加资源和“系统”开始：

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
/// 跟踪按键被按下的时间
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

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

我们大量添加需要增加的东西，我们创建 `Presses` 资源，它保存了一个 `ArrowTimeToml` 列表，以及一个当方向键被按下时添加到该列表的“系统”，并循环所有方向的按键。

我们还需要一个系统来监听 `AppExit` 事件，并将 `ArrowTimeToml` 列表保存到文件中：

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

我们得做点什么来提高这个模式的易用性。当玩家按下一个按键时，相应的方向会有箭头出现在屏幕上。我们将添加两个系统，一个生成箭头，一个切换箭头的可见性：

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

第一个“系统”非常类似于 `spawn_target_arrows`，它只是创建精灵，并添加我们刚刚声明的 `MapMakerArrow` 组件。第二个系统是 `toggle_map_maker_arrows`，根据箭头对应的方向键是否被按下来设置箭头的可见性。我们通过设置精灵的 `Visible` 中的 `is_visible` 字段来做到这一点。

这里有一个问题，我们目前给 `Directions` 声明的 `key_just_pressed` 方法使用了 `just_pressed`，这只会在按键被按下的第一帧时才会生效。我们希望玩家按下按键，箭头就立即显示，所以我们添加了另一种 `pressed` 方法，它可以实现我们想要的：

```rust
// types.rs
impl Directions {
    [Other methods...]

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

这样，我们的 `toggle_map_maker_arrows` 系统就可以正常工作了！我们还要给所有的歌曲地图实现一个插件：

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

要想让它运行起来，我们还需要在 `main.rs` 中加上“系统”的调用代码：

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

现在，我们可以运行游戏来看看地图制作模式是否能正常工作：

>[视频资源](https://caballerocoll.com/images/rhythm/rhythm_map_maker_mode.mp4)

请记住，在游戏终端中使用 ESC 键退出，而不是 `Ctrl+C` 键，这样才能保存文件成功。

这是我们得到的一个文件示例：

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

现在我们可以将它添加到 `assets/songs` 目录下，添加 `name` 和 `filename` 字段，这样就有了歌曲的工作地图！

我们需要做的最后一件事是在地图制作模式下播放歌曲，否则它就显得有点鸡肋。我们简单实现一下，并且给使用的歌曲路径硬编码，这样可以让教程简短一些（如果还算短的话）。我们将使用路径 `assets/map_maker_song.mp3` 中的歌曲。玩家必须在地图制作器中修改文件路径来更换歌曲。每个人都可以实现一些自己的“系统”，以更容易地选择地图制作器中使用的歌曲。

## 在地图制作器中播放歌曲
为了让音乐进入地图制作器，我们先要添加一个资源来保存 `Handle<AudioSource>`。我们要为该资源实现 `FromResources`，这样可以在开始时就加载它，当把它加载到地图制作器中时，它就准备好可以玩了：

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

这一次我决定使用一个元组结构体来处理资源，因为我们只有一个字段。`FromResources` 实现了静态资源服务器，它可以加载音频资源。

在那之后，我们要创建一个新“系统”来进行播放音频，我们将把它设置为进入 `MakeMap` 的状态时执行：

```rust
// map_maker.rs
fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}
```

我们要做的最后一件事是将这两个资源加到插件中：

```rust
// map_maker.rs
pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .init_resource::<MapMakerAudio>() // <--- 新代码
            .on_state_enter(APP_STATE_STAGE, AppState::MakeMap, start_song.system()) // <--- 新代码
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

找一个音频文件，并将其放到 `assets/map_maker_song.mp3` 中，如果你运行游戏，进入地图制作模式时，应该可以听到音频播放了！

至此，我们的游戏教程就结束了。和往常一样，你可以随意尝试，修改一些东西，让它成为你的东西！如果你有任何的改进，请在 [Twitter](https://twitter.com/guimcaballero) 标记我，这样我就能看到了！

## 下一步
如果你还没想好要做什么样的二次开发，以下提供一些可以尝试的想法：

* 1.添加必须在特定的时间内保持状态的箭头。
* 2.改进地图制作器，增加选择歌曲的功能。
* 3.给游戏增加一个游戏结束画面。
* 4.增加一种歌曲播放完后，回到菜单的方式
* 5.创建一个可以改变点击阈值的“系统”，可以让玩家在困难模式时选择简单模式，玩家很轻松则切换到困难模式。

