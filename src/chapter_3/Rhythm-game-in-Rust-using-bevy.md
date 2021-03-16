>* Rhythm game in Rust using Bevy 译文（基于 Rust 用 Bevy 实现节奏大师游戏）
>* 原文链接：https://caballerocoll.com/blog/bevy-rhythm-game/
>* 原文作者：[Guillem Caballero Coll](https://github.com/guimcaballero)
>* 译文来自：[RustMagazine 2021 期刊](https://github.com/RustMagazine/rust_magazine_2021)
>* 译者：[suhanyujie](https://github.com/suhanyujie)
>* ps：水平有限，翻译不当之处，还请指正。
>* 标签：Rust, Bevy,  game, Rhythm game

>2021/2/8 - 77 min read

## Introduction
In this tutorial we'll use the [Bevy](https://bevyengine.org/) game engine to make a rhythm game in Rust. The objective is to show off how to do things in Bevy, specially some more advanced features, like shaders, states, and audio.
>在这个教程中，我们使用 Bevy 引擎用 Rust 实现一个节奏大师游戏。目的是展现如何用 Bevy 实现，特别是一些更高级的功能，如着色器，状态，和音频。

If you want to see the final code before diving in, you can find the repository [here](https://github.com/guimcaballero/bevy_rhythm), and here's a video of how the game works:
>如果你想在进入学习之前看看最终的代码，你可以在这里找到仓库，并且下面是一个操作该游戏视频：

<iframe height=498 width=510 src="https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4" frameborder=0 allowfullscreen></iframe>

This game will be pretty simple: arrows will be flying through the screen, and the player has to press the correct key at the right time to make it disappear. If they do so successfully, they'll gain points. If they don't, the arrow will fall down spinning. Arrows will have different speeds, each of them with a different color. The game will have a menu to select songs, and a small map maker to help create maps for the songs.

## Bevy
[Bevy](https://bevyengine.org/) is a data-driven game engine built in Rust. It's really straight forward to use, and a joy to work with. It uses [ECS](https://en.wikipedia.org/wiki/Entity_component_system) to manage the games entities and their behaviors.

Bevy has a very welcoming community, so if you have any doubts during this tutorial, check out the [Bevy book](https://bevyengine.org/learn/book/introduction/), look through the [examples](https://github.com/bevyengine/bevy/tree/master/examples), or join the [Official Discord](https://discord.gg/gMUk5Ph) to ask questions!

If you find any problem in this tutorial, please open an [Issue](https://github.com/guimcaballero/bevy_rhythm/issues) here so I can fix it.

## Prerequisites
For this tutorial you'll need to be familiar with Rust. You don't need to be an expert, we're not going to use any black magic. Some knowledge of how ECS works is strongly recommended, although you might be able to do without it.

If you want to first checkout some lighter tutorials, I recommend reading [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/) or my [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial/) tutorials, which go over the basics in more detail.

Also, we'll be playing around with shaders and [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language) in this tutorial. Knowledge of either is not necessary, as I'll provide the code we're going to use, but knowing GLSL will allow you to change things and make the game yours!

Here are some recommendations to get started if you've never worked with shaders before:

* [Shadertoy for absolute beginners](https://www.youtube.com/watch?v=u5HAYVHsasc): Introduction to shaders and using [Shadertoy](https://www.shadertoy.com/).
* [Intro to Shader Coding in Unity - An Improvised Live Course](https://www.youtube.com/watch?v=9WW5-0N1DsI): Introduction to using shaders in Unity. Most of the non-Unity specific knowledge should be applicable here.
* [Unity Tutorial: A Practical Intro to Shaders - Part 1](https://www.youtube.com/watch?v=C0uJ4sZelio): Same as above.

## Creating a project
As always, let's start by creating an empty Rust project with `cargo new bevy_rhythm && cd bevy_rhythm`. You can now open `Cargo.toml` with your preferred editor, and add `bevy` as a dependency:

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

## Fast compiles
I recommend you enable Fast Compiles, to ensure that the development process doesn't become tedious. Here's what we need for that:

* 1.LLD Linker: The normal linker is a bit slow, so we can swap it out for the LLD Linker to get a speedup:
    * Ubuntu: `sudo apt-get install lld`
    * Arch: `sudo pacman -S lld`
    * Windows: `cargo install -f cargo-binutils and rustup component add llvm-tools-preview`
    * MacOS: `brew install michaeleisel/zld/zld`
* 2.Enable nightly Rust for this project: rustup toolchain install nightly to install nightly, and rustup override set nightly on the project directory to enable it.
* 3.Copy the contents of [this file](https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds) into `bevy_rhythm/.cargo/config`.

That should be everything, run the game now to compile all of the libraries. At the end, you should see `Hello, world!` in the command line.

Note: If you see that the performance of the game is bad, or you see [assets taking a long time to load](https://github.com/guimcaballero/bevy_rhythm/issues/1), you can run in release mode with `cargo run --release`. Compile times might be a bit longer, but the game will run much smoother!

## Getting started
The first step for any Bevy game is to add the small boilerplate code to start the app. Open `main.rs`, and replace the existing `main` function with the following:

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

![](https://caballerocoll.com/images/bevy_empty_window.png)

This step sets up a Bevy `App`, adding the default plugins. This will include things like transforms, input, windows and everything else we need to get a game going. In case you don't need some of these features, Bevy is modular enough to allow you to select which ones you want to enable. We'll be using all of them, so we use `add_plugins` with `DefaultPlugins`.

We've also added the two resources: `Msaa` and `WindowDescriptor`, to configure anti-aliasing, and the window size and title respectively. Last thing we added is the `exit_on_esc_system` that comes with Bevy, which will take care of closing the game when we press the escape key.

## ECS in Bevy
Here's a small introduction to how ECS works in Bevy. Feel free to [skip this chapter](https://caballerocoll.com/blog/bevy-rhythm-game/#adding-a-setup-system) if you already know how it works. This is completely unrelated to our game, I'll be using examples from the [Bevy book](https://bevyengine.org/learn/book/getting-started/ecs/) to illustrate how it works. You don't have to copy the code here, just read through it and make sure you understand what's going on.

Bevy's ECS is a fork of [hecs](https://github.com/Ralith/hecs). It uses normal Rust structs as components, without the need to add macros or any complicated stuff. For example, we could have:

```rust
// Component with two fields
struct Position { 
    x: f32,
    y: f32
}

// Tuple component
struct Name(String);

// We can even have marker components
struct Person;
```

Systems are just normal Rust functions, that have access to `Querys`:

```rust
fn set_names(mut query: Query<(&Position, &mut Name), With<Person>>) {
    for (pos, mut name) in query.iter_mut() {
        name.0 = format!("position: ({}, {})", pos.x, pos.y);
    }
}
```

A query allows us to access the entities that have the provided components. In the previous example, `query` allows us to iterate over the `Position` and a `Name` components of entities that have those components and also a `Person` component. As we're using `&mut Name` instead of `&Name`, we can modify it. If we tried to modify it while using the latter, Rust would complain.

Sometimes we might want to have a system that only runs once at the beginning of the game. We can do that with startup systems. Startup systems are declared exactly the same as normal systems, the only difference is how we add them to the game, which will be shown later. Here's an example of a startup system that uses `Commands` to spawn some entities:

```rust
fn setup(commands: &mut Commands) {
    commands
        .spawn((Position { x: 1., y: 2. }, Name("Entity 1".to_string())))
        .spawn((Position { x: 3., y: 9. }, Name("Entity 2".to_string())));
}
```

Bevy also has Resources, which allow us to keep global data. For example, the built-in `Time` resource provides us with the current time in the game. To use resources in a system, we use `Res`:

```rust
fn change_position(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut pos in query.iter_mut() {
        pos.x = time.seconds_since_startup() as f32;
    }
}
```

Making our own resource is easy as well:

```rust
// A simple resource
struct Scoreboard {
    score: usize,
}

// Another resource, this one implements Default
#[derive(Default)]
struct OtherScore(f32);
```

We have two options to initialize resources: the first on is to use `.add_resource` and to provide the struct we want, the other option is to use `.init_resource`, if it implements either the `Default` or the `FromResources` trait.

And here's how we'd add these systems to our game:

```rust
fn main() {
    App::build()
        // First way of adding a resource
        .add_resource(Scoreboard { score: 7 })
        // Second way of adding a resource, will be initialized using Default
        .init_resource::<OtherScore>()

        // Add a startup system, only runs once at the start
        .add_startup_system(setup.system())
        // Add a normal system, runs once every frame
        .add_system(set_names.system())
        .add_system(change_position.system())
        .run();
}
```

Another cool thing Bevy has are Plugins, which we've already seen when we used `DefaultPlugins` in the previous section. Plugins allow us to wrap features that belong together, which then let's us enable and disable them together easily. Plugins also provide organization, which is the main purpose we'll be creating our own in this tutorial.

If there's some stuff that isn't clear right now, don't worry too much, we'll go over all of this in more detail later.

## Adding a setup system
Every game needs a camera to render the objects, so we'll begin by adding a startup system that spawns a camera. Since this is a 2D game, we'll use the aptly named `Camera2dBundle`.

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

Note: We could add all of the components individually instead of using the bundle, by using a tuple like so:

```rust
fn setup(commands: &mut Commands) {
    commands.spawn((Camera::default(), OrthographicProjection::default(), VisibleEntities::default(), Transform::default(), GlobalTransform::default()));
}
```

This code doesn't actually work, as we would need to set up some of the fields in the camera and projection components, but I think it illustrates how using a bundle is similar to adding all of the structs by themselves in a tuple.

## Loading sprites
In this first section we'll be adding some sprites and making them move around. For that, we need to create an `assets` directory, where we'll store the [images](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/images) and [fonts](https://github.com/guimcaballero/bevy_rhythm/tree/main/assets/fonts). We'll have two subfolders inside, images and fonts. You can go to the previous links to download the files from the GitHub repository.

Your assets folder should look like this:

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

With our assets taken care of, we can start coding some behavior. We'll make a new file called `arrows.rs` that will keep all the systems that relate to spawning, moving and despawning arrows. The first thing will be a resource that keeps the materials for the arrow sprites, this way we don't have to load them every time we want to create an arrow:

```rust
use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
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

As you can see, instead of actually holding the `ColorMaterials`, the resource has `Handle<ColorMaterial>`. This way, when we create the arrows, we can give them the same handle, and they'll all share the same materials, instead of each having their own.

## Spawning and moving arrows
The next thing we'll be working on is spawning the arrow sprites and moving them across the screen. We'll start by making a system that spawns an arrow once a second. The arrow will have an empty component called `Arrow`:

```rust
/// Arrow component
struct Arrow;

/// Keeps track of when to Spawn a new arrow
struct SpawnTimer(Timer);

/// Spawns arrows
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

To use a timer, we have to manually call it's `tick` method with the time passed since the last time we called it, and we can then use `just_finished` to see if the timer is done. Effectively, what we're doing is ensure that the `spawn_arrows` system only runs once a second, by having an early return that checks if the timer is done.

The rest of the system creates a `Transform` component, that we'll add to the arrow, and it spawns a `SpriteBundle` to spawn the arrows, giving it the red texture from the `ArrowMaterialResource`. To the arrow we're adding the `Arrow` component using the `with` method in `Commands`. This way, the entity we're creating will have all of the `SpriteBundle` components plus our `Arrow` component.

Note: this system is just temporary, and will be replaced by something that spawns the arrows at certain specified times.

Now, those arrows we're spawning are just standing there, so let's make them move to the right with another system:

```rust
/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}
```

`move_arrows` uses a `Query` to take all of the entities with a `Transform` and an `Arrow` component, and changes their translation to be a bit to the right, by increasing the x coordinate. We're also using `Time::delta_seconds()` to increase the distance according to how much time has passed since the last frame.

We'll join all of these systems, `ArrowMaterialResource`, and `SpawnTimer`, into a Plugin:

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

If you run the game now, you should see arrows flying across the screen:

![](https://caballerocoll.com/images/rhythm/rhythm_red_arrows_moving.mp4)

## Types and constants
There's some values that we have hardcoded in the previous section. As we're going to be reusing them a bit, we're going to make a small module where we'll keep our constants. Make a new file called `consts.rs`, and add the following to it:

```rust
/// Speed at which a Slow arrow moves
pub const BASE_SPEED: f32 = 200.;

/// X coordinate value at which arrows spawn, should be out of screen
pub const SPAWN_POSITION: f32 = -400.;

/// X coordinate value where the arrows should be clicked
pub const TARGET_POSITION: f32 = 200.;

/// Margin of error for clicking an arrow
pub const THRESHOLD: f32 = 20.;

/// Total distance traveled by an arrow, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;
```

Some of these constants won't be used until a bit later. Add `mod consts` in `main.rs`, to import the module and make it available. We can replace those values in `spawn_arrows` and `move_arrows` in `arrows.rs` to use the constant, like so:

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
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * BASE_SPEED;
    }
}
```

We now have arrows moving across the screen, but at the moment they're all facing the same way, going at the same speed and all have the same color. To be able to tell them apart, we'll create two different enums, one for Directions (Up, Down, Left, Right) and one for Speed (Slow, Medium, Fast).

Note: We're calling it `Directions` instead of `Direction`, because the latter is a [Bevy enum](https://docs.rs/bevy/0.4.0/bevy/prelude/enum.Direction.html). By calling it a slightly more awkward name, we're saving ourselves the trouble of having to tell them apart.

Let's create a new file called `types.rs`, where we'll put these enums:

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

`key_just_pressed`, which checks if a key associated to the direction is being pressed. I've decided to add `D, F, J, K` as possible keys too, as the arrow keys on my keyboard are a bit small. Feel free to replace these with whatever else you want, like `W, S, A, D` if you're more of an FPS person, or `K, J, H, L` if you're living the VIM life.

Note: If you're not very comfortable with iterators, here's how `key_just_pressed` would look like using a more traditional approach:

```rust
/// Checks if a key that corresponds to this direction has been pressed
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

That was a bit of code, but nothing too complicated I hope! The next types we're going to add are `ArrowTime` and `SongConfig`. The first one will keep track of when an arrow needs to be spawned, and with which direction and speed. The second will keep the list of all the arrows:

```rust
#[derive(Clone, Copy, Debug)]
/// Keeps track of when each arrow should spawn and it's speed and direction
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

If we run the game now, nothing has changed, but it still works, which is always great! Let's go into arrows.rs and change it so that it spawns arrows according to the list in SongConfig.

## Spawning arrows on time
Now that we have a list of arrows to spawn, we can remove all of our timer stuff and change the `spawn_arrows` system to check what arrows it should spawn each frame.

A first implementation we could come up with would loop through all of the arrows in `SongConfig`, and check which ones should be spawned in the current frame. This would work, but we would be looping over a possibly large list each frame. It's not much of an issue when we only have the 5 arrows we have hardcoded, but a song could be more than a 1000 arrows long, and even though computers are fast, players certainly won't appreciate us needlessly heating up their CPUs.

Instead, we're going to assume that the arrows in `SongConfig` are sorted. We'll need to take care of actually sorting them before starting the song, but that's easy enough. Knowing that, we can check only the first arrow on the list, and if it should be spawned, we also check the next one, repeating until we reach an arrow that doesn't need to be spawned on that frame. Since the arrows are ordered, if an arrow doesn't need to be spawned, neither of the following arrows will either. After that, we'll need to remove the arrows we have spawned from the beginning of the list.

We're also going to add `Speed` and `Directions` as fields for `Arrow`:

```rust
// At the top
use crate::types::*;

/// Actual component that goes on the sprites
struct Arrow {
    speed: Speed,
    direction: Directions,
}

/// Spawns arrows
fn spawn_arrows(
    commands: &mut Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow acording to direction
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
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}
```

That's a bit of code, so let's go through it.

At the beginning of the system we first get how many seconds have passed since the start of the game, and the time when this system last run. We do this using [delta_seconds_f64](https://docs.rs/bevy/0.4.0/bevy/core/struct.Time.html#method.delta_seconds_f64), which returns the time that has passed since the last game update. With this two values, we can know which arrows we should spawn. As Bevy doesn't update our game every nanosecond (not that any game engine does that), simply checking if `spawn_time` is equal to the current time would cause us to skip arrows. For example, we might have an arrow with a spawn time set to `3.0`. Bevy might run this system once at time `2.99` and then at `3.01`. Since our arrow is set to spawn at `3.0`, it wouldn't match any of the times we run the system, and so it would never get spawned.

With our method here, at the beginning of the system we check for the current time, and the last time, so for our example, the second time we run, we would have `secs = 3.01` and `secs_last = 2.99`, and since our arrow's spawn time is over `secs_last` but under `secs`, we do spawn it. Nice!

With that, we can make a small change in `move_arrows` so that it takes speed into account, using the `Speed::value()` method we created before:

```rust
/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();
    }
}
```

Cool, we now have each arrow displayed with it's correct color and moving at the speed it should:

![](https://caballerocoll.com/images/rhythm/rhythm_colored_arrows_moving_at_speed.mp4)

## Adding target arrows
We're now going to use our `border_texture` to create the target arrows, so the players can know when they should press the button. For that, we'll make another startup system, `setup_target_arrows`, and a marker component, `TargetArrow`:

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

Don't forget to add `setup_target_arrows` as a startup system in `ArrowsPlugin`:


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

![](https://caballerocoll.com/images/rhythm/rhythm_target_arrows.mp4)

## Despawning arrows when pressed
Now that we have target arrows, let's implement a system that will despawn the arrows if the correct key is clicked while the arrow is inside the threshold. We'll make a new system, called `despawn_arrows`:

```rust
/// Despawns arrows when they reach the end if the correct button is clicked
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);
        }

        // Despawn arrows after they leave the screen
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
        }
    }
}
```

We use a `Query` to get all of the entities with a `Transform` and an `Arrow` component. We've also added `Entity` to the query, which gives us access to the entity's "id", which we can then use in `Commands::despawn()` to despawn the entity. We then loop through the arrows, and check if the x coordinate is inside the threshold for clicking, and if so, despawns the arrow. It also has a second check, to despawn an arrow after it has been missed and it has left the screen. It's done in a bit of a lazy way, with `2. * TARGET_POSITION`.

Remember to add this system to `ArrowsPlugin` with `.add_system(despawn_arrows.system())`, and with that done, you can run the game and actually kind of play something that when we squint might resemble a game!

## Adding basic UI
In this section we'll implement some basic UI, which for now will just show the current time in the song. We'll keep all of it in `ui.rs`:

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

We can now add a system which updates the text each frame:

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

We'll finish this file by making a plugin:

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

![](https://caballerocoll.com/images/rhythm/rhythm_time_text.mp4)

## Adding scores
In this section we'll make a scoring system, so that players can see how well they did after each run. For that, let's open yet another file, `score.rs`, where we'll create a new resource that will keep track of both the score and the number of correct and failed arrows:

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
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // Get a value from 0 to 1 according to how close the press was
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // Give at least 10 points and 100 at max
        let points = (score_multiplier * 100.).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

    /// Increases number of failures
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

Let's add this resource by going to `main.rs` and adding the following:

```rust
mod score;
use score::ScoreResource;
```

And replace the `main` function with the following:

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

Now, having a scoring system is all very fine and dandy, but it's a bit worthless if the player can't see how well they are doing! Let's make some changes in our UI module to also display the score:

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

We should also make changes in `setup_ui` to spawn a second `NodeBundle` and `TextBundle`, this time with the `ScoreText` component:

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

There we go! Let's run the game to see how our hard work is paying off:

![](https://caballerocoll.com/images/rhythm/rhythm_score_text.mp4)

Feel free to spice up the UI as you wish! What we've done here is just to show the basics of how to display text.

## Loading from toml file
Currently our game's arrows are hardcoded. This has been alright for now, but we would really like it if players can make their own songs. We won't complicate ourselves by making a custom file format or any fancy things, so we'll go with the tried and tested [TOML](https://en.wikipedia.org/wiki/TOML) format, by using the [toml](https://github.com/alexcrichton/toml-rs) and [serde](https://github.com/serde-rs/serde) crates. This two crates together will help us to very easily implement TOML serialization and deserialization for our `SongConfig` struct.

Add the following to `Cargo.toml` file:

```toml
toml = "0.5.8"
serde = "1.0.118"
serde_derive = "1.0.118"
```

We can now go into `types.rs` and start preparing our types for deserialization, by importing some things and adding the `Deserialize` and `Serialize` traits to `Directions` and `Speed`:

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

name will be used to store the song's display name, `filename` will be the path to the audio file, and `arrows` is the list of `ArrowTimeTomls`. `ArrowTimeToml` has the same fields as `ArrowTime`, but it has `click_time` instead of `spawn_time`.

We'll also replace `ArrowTime::new` to instead take an `ArrowTimeToml`:

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

```rust
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}
```

We keep the audio with a `Handle<AudioSource>`, which we'll load using the `AssetServer` when we transform the `SongConfigToml` to `SongConfig`.

Finally, we'll change `load_config` to load a `SongConfig` from a file:

```rust
pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // Open file and read contents
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    // Parse using toml and Serde
    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

    // Process arrows
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<ArrowTime>>();
    // Sort arrows by spawn_time
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // Load song audio and get the handle
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}
```

That's a few lines of code, but it's pretty straight forward: we first open the file and read it's contents, we parse it using toml's `from_str` function, then change the vector of `ArrowTimeTomls` into a vector of `ArrowTimes`, we load the song's audio using `AssetServer::load`, and then return the newly built `SongConfig`.

Note: `AssetServer::load` will search for the file inside the `assets` folder. `File::open` will instead search on the root folder, so we have to manually add `assets` at the beginning of the path.

We'll also need to change the `setup` system in `main.rs` to take `AssetServer` as a parameter, and we have to change the call to `load_config`:

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

Your assets folder should look like the following:

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

## Playing audio
A thing you might have noticed, is that in last section we implemented something to load the song audio, but it still doesn't play when we're playing the game. Let's implement that now! For that we'll open a new file, `audio.rs`, which will contain just one system:

```rust
audio.rs
use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, config: Res<SongConfig>) {
    // Song starts 3 seconds after real time
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

Note: We could have used a `Timer` that doesn't repeat, but this will make it more complicated later, when we make a menu to select the song. It can be a good exercise to try to rewrite it using timers though!

On `main.rs`, we should add the following:

```rust
// main.rs
mod audio;
use audio::AudioPlugin;
```

And in the `main` function, after all the other plugins, add `.add_plugin(AudioPlugin)`. Running the game now should result in the song starting to play as the timer starts running!

With this we have concluded implementing the core gameplay for our game. You can feel free to branch off into building your own thing on top of the base we've built here, but I recommend you stick around a bit longer, as we'll be working on making things ✨ fancier ✨.

## Fancy failing arrows
For starters, we could improve how failed arrows look. Currently, they just fly off into the distance. We'd ideally like to give the player a little more indication that they have failed.

What we're going to do is to have the arrows "fall off" their line after passing the target. To implement that, let's add some things to `move_arrows` in `arrows.rs`:

```rust
/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        // New
        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;
        }
    }
}
```

What we're doing is get the signed distance from the target to the arrow only in the `x` coordinate, and if it's positive, meaning that it's moved past the target, we substract a bit to its `y` coordinate, so it goes down. By using `time.delta_seconds() * distance_after_target`, we make the lowering factor bigger each frame, which will make it fall down in an arc. The `2`. is just a magic constant to make the arc nicer (for me), feel free to adjust it to your taste!

Here's how this looks:

![](https://caballerocoll.com/images/rhythm/rhythm_arrows_falling.mp4)

That's good, but let's give it a bit more effect. We'll make the arrows also shrink and spin as they fall:

```rust
/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // Move the arrow down if it's past the target
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;

            // Change the scale according to how far away the arrow is
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            // Rotate the arrow according to distance and speed
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}
```

This is a bit full with magic numbers mixed with formulas I came up with after trying different things. I encourage you to try playing with it and making something else!

Let's break it down a bit: first, we get a scale using a formula which decreases as the arrow moves away. We then use `max` to ensure that the scale is at least `0.2`. After that, we use [Transform::rotate](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.Transform.html#method.rotate) to rotate the arrow. For the rotation we use `Speed::multiplier`, to have the arrows spin faster if they have a faster speed. Here's how all of these look together:

![](https://caballerocoll.com/images/rhythm/rhythm_arrows_spinning.mp4)

Ayyy that's pretty cool! Again, feel free to improvise and add other things that make it look better. Half of the fun comes from making fancy things you enjoy looking at!

## Shader backgrounds
Next thing we'll work is replacing the gray background. One option would be using the `ClearColor` resource to have a static color as background. [Here](https://github.com/bevyengine/bevy/blob/v0.4.0/examples/window/clear_color.rs)'s an example of how it's used. It's pretty simple, we'd just have to add `.add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))` in `main`, but this only allows us to change the background to a flat color, and we would preferably like something more animated. Shaders to the rescue!

We'll make a sprite the size of the window under everything, and we'll add a shader material. This way, we'll have a background where we can set a shader as the background.

As we'll be adding some other stuff with shaders, we'll make a folder called `shaders` where we'll keep all of our things. Let's open `shaders/mod.rs`:

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

We're making use of the `WindowDescriptor` resource to access the screen width and height, so that we can set the transform correctly. There's going to be a slight issue if the player makes the window bigger, as our background will stay the same size, and that will show the gray background behind! To fix this, we'll add another small system:

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

As you may have noticed, there's a pattern of things being easy and pleasant to use in Bevy. Events are no different. To use an event, we need to add an `Event<T>` resource and a `Local<EventReader<T>>` as parameters. We can then use `EventReader::iter` by providing it the event resource, which will provide us the events that we haven't processed yet.

The actual shaders are added using Rust's `include_str` macro, which will add the contents of the file as a string. First we'll make `background.vert`:

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

We now need to register these systems we have created. Let's add `ShaderPlugin` in `shaders/mod.rs`:

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

![](https://caballerocoll.com/images/rhythm/rhythm_simple_background.mp4)

## Using time in shaders
We have some kind of fancy background going on, cool! Ideally though, we'd like to have it change through time.

Bevy doesn't (at least not for now) add the time nor the resolution into shaders as an input, so we'll have to manually add them ourselves. Hopefully this will be improved soon.

Let's open `shaders/mod.rs` again and add the following code:

```rust
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
/// Resource that will be passed to shaders
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

/// Updates time in ShaderInputs every frame
fn update_time(time: Res<Time>, mut nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    for mut node in nodes.iter_mut() {
        node.time = time as f32;
    }
}

/// Updates resolution in ShaderInputs if window size changes
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
fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}
```

We're making a new struct called `ShaderInputs`, which we add as a render graph edge in `setup_render_graph`, which we'll add as a startup system. `update_time` and `update_resolution` are two systems that take care of updating the values for each entity that has the `ShaderInputs` component. Notice how on `update_resolution` we're listening to the `WindowResized` event instead of updating every frame.

Now, replace `ShaderPlugin` with the following, to add all of these systems and assets:

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

```rust
// shaders/background.rs
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
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

Here's how this looks like:

![](https://caballerocoll.com/images/rhythm/rhythm_background_with_time.mp4)

Personally, I've settled on the following shader as a background:

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

![](https://caballerocoll.com/images/rhythm/rhythm_fancy_background.mp4)

Now it's your turn to play around with it and find something you like. If you don't feel too confident with shaders, you can try making slight modifications to the one above, or you can go to [Shadertoy](https://www.shadertoy.com/) and find inspiration from there. For example, the following is a [shader](https://www.shadertoy.com/view/XsXXDn) by Danilo Guanabara, translated from Shadertoy:

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

![](https://caballerocoll.com/images/rhythm/rhythm_creation_background.mp4)

## Fancy click animation
We've previously added a fancier animation for when an arrow is failed, but we still do nothing when an arrow is correctly clicked. It just disappears, which is slightly disappointing. Let's work on improving that.

We're going to have four different sprites, each with a shader material, under each of the target arrows. Then, we'll make it so each time an arrow is correctly clicked, the shader of the corresponding sprite starts the animation, which will last some time and then disappear.

Note: The way this is going to be implemented is a bit more complex than what we could technically do, but it allows me to show some more stuff. An easier way to implement this would be to create a sprite each time an arrow is correctly clicked, and then removing it after some seconds.

We'll open a file named `shaders/target_arrows.rs`, where we'll add a component for these sprites (which I'm calling "target arrow sparkles"), which only keeps the direction of the target arrow it's associated to:

```rust
pub struct TargetArrowSparkle {
    direction: Directions,
}
```

We'll also add another edge to the render graph, with another struct we want to pass to the shader as parameter. This will keep the last time that there was a correct arrow, and how many points it was worth:

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

Now, let's add a startup system to create the sprites:

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

We've set `last_time` to 3 seconds, to test. This will make it so that when the time reaches three, the animation starts. When we have everything properly set up we'll change it to a negative value, as we don't want the animation going off until an arrow is correctly clicked.

We also need to create new files for the shaders:

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

We also need to add `setup_target_arrows` to `ShaderPlugin`:

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

![](https://caballerocoll.com/images/rhythm/rhythm_half_done_fancy_clicking.mp4)

As you can see, right when the song starts, at 3 seconds, all of the circles start growing, and a bit after half a second they disappear. This is great news, it means that both the shader and the time are working! We're still missing something to update the value though, so let's add a system to update the `last_time` value when an arrow is correctly clicked. Before that, let's set the default value to be something negative:

```rust
// shaders/target_arrows.rs
.with(TimeSinceLastCorrect {
    last_time: -10.,
    points: 0.,
})
```

If you run the game now, the circles shouldn't appear at all.

Previously, we've seen how to listen to events, but we still haven't looked at the other side of the coin. We'll be working with sending them now. We'll make an event that is sent when an arrow is correctly clicked. We'll send this event from inside the `despawn_arrows` system, in `arrows.rs`:

```rust
// arrows.rs
/// Event struct
pub struct CorrectArrowEvent {
    pub direction: Directions,
    pub points: usize,
}

/// Despawns arrows when they reach the end if the correct button is clicked
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
            correct_arrow_events.send(CorrectArrowEvent {
                direction: arrow.direction,
                points,
            });
        }

        // Despawn arrows after they leave the screen
        if pos >= 2. * TARGET_POSITION {
            commands.despawn(entity);
            score.increase_fails();
        }
    }
}
```

The first thing we've done is create a new `CorrectArrowEvent` struct, which will be our event. To `despawn_arrows`, we've added a `ResMut<Events<CorrectArrowEvent>>` parameter, so that we can `send` events through it, using the send method. To send an event we need to pass a `CorrectArrowEvent` struct, with the direction of the arrow and the points that the player got.

We now need to add `.init_resource::<Events<CorrectArrowEvent>>()` to `ArrowsPlugin`, and we're ready to go. Easy, right?

We're now going to add a system in `shaders/target_arrows.rs` which takes care of updating `last_time` in the correct target arrow sparkles:

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

Add this last system to `ShaderPlugin`, `.add_system(correct_arrow_event_listener.system())`. If you run the game now, you'll get the circles when you correctly click an arrow:

![](https://caballerocoll.com/images/rhythm/rhythm_target_arrow_circles.mp4)

That's all of the shader-ing we'll do in this game. As always, feel free to change things up, add more effects, and experiment!

## Adding states
In the next sections we'll work on making a very simple song select menu. For that we'll be working with States, which will require some changes all over the place. To make a State, we need to create a new enum, and add it as a resource wrapped in a [State](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.State.html). Then, we can assign each system to a specific state, using `on_state_update`, `on_state_enter`, and `on_state_exit`.

Let's start working on this. First, let's open `consts.rs` and add our state enum:

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

We're also adding a string for the stage we'll use for our systems. Now we'll go into `main.rs` and add both the `State` resource, and our new stage after Update:

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

Let's now go to `ui.rs`:

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

## Adding a basic menu
We'll make an actual menu with buttons now, that allows us to either select a song or to enter map maker mode. We'll keep it all in a new file, `menu.rs`. We'll start by making a resource to keep the materials:

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

We'll also make a system that removes all of the buttons, so that we can run in when we leave the `Menu` state. If we didn't, the buttons would still stay over the screen on `Game` mode.

```rust
// menu.rs
fn despawn_menu(commands: &mut Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}
```

Let's also make a plugin for these systems:

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

Running the game now shows the following Play button:

![](https://caballerocoll.com/images/rhythm_basic_menu.png)

Currently, clicking and hovering over the button doesn't do anything, so let's work on making the menu more reactive. First, we'll add a system that changes the color according to the interaction state of the button:

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

![](https://caballerocoll.com/images/rhythm/rhythm_button_interactions.mp4)

## Improving our menu
We still need two things: the menu to show the list of songs inside the folder, and the buttons to actually start the game. Let's start with the first of those, by adding another method in `menu.rs`:

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

This function uses [read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html) to get the files inside the `songs` folder, and adds the ones ended in `.toml` to a vector.

We can now call this function from inside `setup_menu` to add a button for each one of the files we get from `get_songs`. First, we'll make an enum component to add to our buttons:

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

Note: The way we've setup the buttons is very naive, so if you have a lot of buttons to display, it's going to look weird! A system for scrolling the buttons or anything to improve how the menu looks is left as an exercise for the reader.

Here's the result:

![](https://caballerocoll.com/images/rhythm_menu_with_correct_buttons.png)

Let's now work on making the buttons usable. For that, we'll add another system that will listen to clicks:

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
        if *interaction == Interaction::Clicked {
            match button {
                // If it's the map maker button, change the state
                MenuButton::MakeMap => state
                    .set_next(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                // If it's a play song button, load the config, insert it, and change state
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

Finally, we should add a this system to `MenuPlugin`, set to run on update with the `Menu` state:

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

![](https://caballerocoll.com/images/rhythm/rhythm_menu_changing_state.mp4)

But there's a big issue! The arrows aren't appearing, and the time is already running when we get to the game! Since we're using `time_since_startup` to check when to spawn the arrows, when we enter the `Game` state, the value is already past the first arrow's spawn time, so it doesn't appear, and none of the others do either. To fix that, we'll make a wrapper over time, so that we can reset it when we enter `Game` mode.

## Wrapping time
Our time wrapper will be very similar to Bevy's normal implementation of the Time resource, but it will have a system that resets time when we enter the `Game` or `MakeMap` states. It will look a bit bad to copy a all of the code just to be able to change something, but this would allow us to do other time related things in the future, like for example pausing. It's also a good excuse to look at some of Bevy's internal code.

Also, by having both a the normal time resource and our wrapped version, it allows us to use the normal non-resetting time for things, and the controlled time for some others. For example, we'll keep using the normal time for the background, as we want it to work during all the states.

Let's open a new file, `time.rs`:

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

Now, we'll add the methods we'll be using, taken from [the source](https://github.com/bevyengine/bevy/blob/3b2c6ce49b3b9ea8bc5cb68f8d350a80ff928af6/crates/bevy_core/src/time/time.rs), but we'll also add a `reset_time` function, that set's the time to 0:

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
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }
}
```

With that out of the way, we'll need a system that updates the time:

```rust
// time.rs
pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}
```

And another system that resets it:

```rust
// time.rs
pub fn reset_time_when_entering_game(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}
```

We'll also add a Plugin to keep them all together:

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

Like we've done with all other plugins, let's go into `main.rs` to add it:

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

First one will be `ui.rs`, where we just need to change the `time` parameter in `update_time_text`:

```rust
// ui.rs
use crate::time::ControlledTime;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    [...]
}
```

The same thing happens with `audio.rs`, where we'll just replace `Time` for `ControlledTime`:

```rust
// audio.rs
use crate::time::ControlledTime;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    [...]
}
```

Last one is `arrows.rs`, where we do need to make a couple more changes:

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

![](https://caballerocoll.com/images/rhythm/rhythm_working_menu_and_game.mp4)

Awesome!

## Adding a simple map maker mode
In this section we'll add a mode to help us create maps for our songs. What we want is the song to play while we press the keys, and to collect those and save them in a file.

Let's open a new file called `map_maker.rs`, where we'll start by adding a resource and a system:

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
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

/// Saves key presses to Presses
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

We're going to also need a system that listens to the `AppExit` event, and saves the `ArrowTimeToml` list into a file:

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

There's one issue, the `key_just_pressed` method we have currently declared for `Directions` uses `just_pressed`, which will only be true the first frame the key is being pressed. We want our arrows to show up for as long as the player has the key pressed, so we'll add another method that uses `pressed` instead, which does what we want:

```rust
// types.rs
impl Directions {
    [Other methods...]

    /// Checks if a key that corresponds to this direction is being pressed
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
        .add_plugin(MapMakerPlugin) // <--- New
        .run();
}
```

We can now run the game to see our map maker mode working correctly:

![](https://caballerocoll.com/images/rhythm/rhythm_map_maker_mode.mp4)

Remember to exit using the ESC key, and not by `Ctrl+C` in the terminal, so that the file is saved successfully.

And here is an example of the file that we'll get:

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

The last thing we need is to also play the song on the map maker mode, otherwise it's a bit useless. We're going to go the easy route, and we'll hardcode the path for the song used, this way this tutorial doesn't become unbearably long (if it hasn't already). We'll use the song at the path `assets/map_maker_song.mp3`. The player will have to change the file at that path to change the song used in the map maker. Feel free (and I actually encourage) everyone to make some kind of system to select the song used in the map maker more easily.

## Playing songs in map maker
The first thing we'll do to get music going in map maker is to add a resource that holds the `Handle<AudioSource>`. We'll implement `FromResources` for this resource, so that we can load it at the start and it's ready to play as soon as we load into the map maker:

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

After that, we'll make a new system that starts playing the audio, and we'll set it to run on state enter for `MakeMap`:

```rust
// map_maker.rs
fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}
```

The last thing we need is to add these two things to the plugin:

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

With this, we've reached the end of our adventure. As always, feel free to experiment, change things up, and make it yours! If you make any changes, please tag me on [Twitter](https://twitter.com/guimcaballero) so I can see it!

## Next steps
Here are some ideas you could try if you don't have a specific game of your own you want to work on:

* 1.Add arrows that have to be held during a certain amount of time.
* 2.Improve map maker, adding something to select the song.
* 3.Add an end screen to the game.
* 4.Add a way to go back to menu after song has finished.
* 5.Make a system that changes the threshold for clicking so that it's more relaxed if the player is having a hard time, and it's stricter if the player is doing good.
