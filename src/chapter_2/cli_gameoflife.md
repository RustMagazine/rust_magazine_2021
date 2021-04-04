---
pub_date: Sat, 27 Feb 2021 16:00:00 GMT
description: Use Rust to implement a command-line game of life

---

# 「译」使用 Rust 实现命令行生命游戏

译者：m1zzx2 

---

原文：

- [https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-1-57pp](https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-1-57pp)
- [https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-2-16j3](https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-2-16j3)


## 介绍


你好！如果你看到了这篇文章，说明你对Rust感兴趣，并且想学习或者了解它。我早在2020年6月就编写了我的第一个Rust教程[Rust + Actix + CosmosDB (MongoDB) tutorial api](https://dev.to/jbarszczewski/rust-actix-cosmosdb-mongodb-tutorial-api-17i5)。这次，我将尝试介绍Rust的CLI。为了让这次的介绍更有趣，使用了[Official Rust WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/rules.html)教程来实现“生命游戏”，来增强用户的交互逻辑。

虽然这是个新手教程，但是我仍然强烈建议你通过了官方的新手教程后再来做这个。
[rustlings tutorial](https://github.com/rust-lang/rustlings)


可以在我的[github仓库](https://github.com/jbarszczewski/cli-game-of-life)中找到“最终”代码

## 创造Universe
开始吧！
在创建一些新的项目像 new cli-game-of-life (或者 cargo init 如果你已经在一个正确的目录里面)之后。 使用你喜欢的编辑器打开它，目前要忽略main.rs。我们先要创建一个逻辑模块，所以继续创建一个src/game.rs文件。和前面说的一样，我将使用和wasm官方教程一样的逻辑来讲解，如果你之前做过它，你就会对它非常熟悉。让我们在游戏Universe里面来定义一个游戏单元格的枚举。

```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
```
derive 声明会告诉编译器提供(Copy, Clone, Debug, Eq, PartialEq)的基本实现，所以我们可以给单元分配枚举值并且比较他们。

**注意**: 我们也可以用bool值来实现一样的功能，不过使用enum可以具有更好的可读性，两者占用的内存是相等的。

我们的游戏Universe定义如下:
```
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
```

好了现在我们开始实现游戏的函数了。让我们从一个方便的构造函数开始，这个构造函数将会设置Universe的大小，并初始化Cells的初始值。set_cells函数将会接受一个cells坐标，并把对应坐标的Cell设置成Alive状态。

```
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
```
get_index 函数是一个辅助函数，它会把Universed的坐标翻译成cells数组对应的下标。

接下来，我们会实现Display特性，方便打印当前游戏的状态。

```
use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
```

Perfect! Now we have something to run. Head over to your main.rs and replace all with the following content:
非常完美!现在我们需需要定义一个启动函数: 挑转到main.rs 用下面的内容替换main.rs的内容：
```
mod game;

fn main() {
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    print!("{}", game);
}
```

运行 cargo run之后 ,代码顺利的跑起来了，但是它实际上没有做什么，因此我们需要新增一个tick函数:

```
pub fn tick(&mut self) {
    let mut next = self.cells.clone();
    for row in 0..self.height {
        for col in 0..self.width {
            let idx = self.get_index(row, col);
            let cell = self.cells[idx];
            let live_neighbours = self.live_neighbour_count(row, col);
            next[idx] = match (cell, live_neighbours) {
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                (Cell::Dead, 3) => Cell::Alive,
                (otherwise, _) => otherwise,
            };
        }
    }
    self.cells = next;
}

fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
        for delta_col in [self.width - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbour_row = (row + delta_row) % self.height;
            let neighbour_col = (column + delta_col) % self.width;
            let idx = self.get_index(neighbour_row, neighbour_col);
            count += self.cells[idx] as u8;
        }
    }

    count
}
```
该代码直接来自WASM锈皮书，它将Conway的《生命游戏》规则应用到我们的宇宙中，同时还要注意边缘包裹，以使我们的宇宙看起来像是循环的（请参见风味3）。
在使用刻度之前，我们需要准备终端以显示动画游戏Universe。 让我们现在就跳进去！

P.S. -您可以在我的GitHub上找到本章的源代码

这段代码来自wasm rust book ，它把ConWay的 Conway's Game Of Life 的规则应用到我们的universe中，它也会注意边界条件，让我们的universe看起来是循环运动的。[看第三章](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)

在我们使用tick函数之前，我们需要准备用终端去展示Universe 的界面，让我们来进入这个操作吧!

P.S -你们可以在[这里](https://github.com/jbarszczewski/cli-game-of-life/tree/42c60e1c10073dd65819af7d1a6d7b049d1a449d)找到本章的源代码

## 绘制游戏Universe


为了让终端输入输出，我们将会使用[Crossterm crate](https://crates.io/crates/crossterm)包，因此我们需要把它添加进我们的Cargo.toml文件里面:
````
[dependencies]
crossterm = "0.19.0"
````
这个工具箱里面有很多方便的函数来操作终端，并且它是跨平台的，我们不需要担心任何平台的区别。大多数crossterm指令是容易理解的，因为他们被分进了不同的模块，就像cursor:Hide 就是和它的字面意思的一样，隐藏光标。
````
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::stdout;
use std::time::Duration;
````

接下老，我们的main函数需要被填充成这个样子:
````
fn main() -> Result<()> {
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&game),
                Print("Press enter to exit...")
            )?;
            game.tick();
        }
    }
    execute!(stdout(), ResetColor, Show, LeaveAlternateScreen)?;
    Ok(())
}
````
好的让我们拆解一下在做的事情:
1. main函数现在返回了Result类型。这能让用户随时退出。
2. 我们在execute!宏里面设置临时终端，它的第一个参数是std::io::Writer(这个case里面的输入)类，后面的参数是一些命令。
3. 在这个循环里面，我们用poll去读取用户的输入，这样不会阻塞execution去绘画终端。当用户输入回车按钮时，这个循环就会退出，如果用户在500ms内没有输入，我们将会根据tick计算的状态重新绘画Universe。
4. 循环结束以后，我们就会离开这个临时终端。
现在我们可以跑脚本cargo run 了。
你将会看到水平线和垂直线相互交替出现，但是输入enter，游戏没有停止。我们需要修改代码来实现这个功能。

## 和Universe交互
我们只能处理回车的原因是，默认的输入是在按下回车后处理的。通常，你的输入都准备好之后，在按下会车触发，这才有意义。但是在我们的需求里面，我们希望和一个键交互。这意味着我们需要启用[raw mode](https://docs.rs/crossterm/0.19.0/crossterm/terminal/#raw-mode). 新的代码会被改成这样:
```

// add required imports:
use terminal::{disable_raw_mode, enable_raw_mode};

// add this line at the very begining of the main() function:
enable_raw_mode()?;

// replace code block when poll returns true, the match statement, with following:

if let Event::Key(KeyEvent { code, .. }) = read()? {
    match code {
        KeyCode::Esc => {
            break;
        }
        _ => {}
    }
}

// finaly disable raw mode at the end of the function before returning Ok(()):
disable_raw_mode()?;
```
添加循环退出功能是很重要的，因为raw mode模式下，会禁用ctrl+c退出的方式。
现在你可以运行这个代码了，但是你会发现输出的格式都是乱的，这是因为raw mode不会处理换行符。现在我们需要将光标显示在正确的位置。这意味着我们不能用Display 特征来显示了。取而代之的，我们会遍历Universe，把每一行分别打印出来，向Universe中添加新方法：
```
pub fn row_as_string(&self, row: u32) -> Option<String> {
    if row < self.height {
        let mut row_string = String::new();
        let start = self.get_index(row, 0);
        let end = self.get_index(row, self.width);
        let line = &self.cells[start..end];
        for &cell in line {
            let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
            row_string.push(symbol);
        }
        Some(row_string)
    } else {
        None
    }
}
```
如果该行和Universe大小一致，我们返回整行作为一个字符串，否则，返回None.
在我们的main.rs中，从crossterm队列中添加新的导入，请排队！宏类似于执行，但需要手动刷新。如果要有条件地构建输出，这将非常方便。让我们看看它如何进行。首先在main（）函数的开头初始化一个新变量：

```
let mut stdout = stdout();
```
现在，可以把stdout()替换为我们的新名称，我们需要用以下代码替换整个循环:
```
loop {
    if poll(Duration::from_millis(500))? {
        if let Event::Key(KeyEvent { code, .. }) = read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    } else {
        queue!(stdout, Clear(ClearType::All))?;
        let mut i = 0;
        while let Some(line) = game.row_as_string(i) {
            queue!(stdout, MoveTo(0, i as u16), Print(line))?;
            i += 1;
        }

        queue!(
            stdout,
            MoveTo(0, (i + 1) as u16),
            Print("Press Esc to exit...")
        )?;
        stdout.flush()?;
        game.tick();
    }
}
```
按键处理逻辑不会改变,所有的更改都在else里面：
1. 我们把execute!替换成 queue! 宏。
2. 遍历Universe的每一行，queue! 会直接打印结果，你会看到返回Option <T>有多方便！我们不需要任何额外的处理，这个代码看起来会很干净。
   
3. 在所有文本都准备好之后，我们调用 flush() 刷新到输出。
## 接受参数
使用std :: env :: args函数可以非常简单的接受参数.但是我想展示一些依赖外部包 [clap](https://crates.io/crates/clap)的方法。有三种配置clap的方式:

- 'Builder Pattern'
- yaml配置
- 宏
'Builder Pattern'是我最喜欢的一种方式，它可以动态扩展输入的参数，并提供一些检查。对于像这样的简单项目，将配置放在main.rs中是完全可以的，随着项目复杂度的增长，可能湖考虑把配置放在单独的文件里面，可以有更好的可读性。首先Cargo.toml添加依赖：
```
clap = "2.33.3"
```
接下来更新我们的main.rs文件:
```
use clap::{crate_version, App, Arg};

//below code goes at the beginning of main() function:
  let matches = App::new("CLI Game Of Life")
        .version(crate_version!())
        .author("jbarszczewski")
        .about("Simple implementation of Conway's Game Of Life in Rust.")
        .after_help("Have fun!")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to configure initial state of game")
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DELAY")
                .help("Sets the delay between game ticks. Value is in miliseconds")
                .short("d")
                .long("delay")
                .takes_value(true)
                .default_value("500"),
        )
        .get_matches();
```

 clap包会创建两个子命令(除非你覆盖了它们):

- help (-h or --help)
- version (-V --version) That's why we provide basic info about the app. You may notice crate_version! macro, this will grab the version number from your Cargo.toml file so you don't need to manually update it. Then we add two arguments, INPUT and DELAY, with some description how to use it. Build your app with cargo build (you will find binary in /target/debug directory) and run like this ./cli-game-of-life -h which will print out help page:
```

CLI Game of Life 0.2.0
jbarszczewski
Simple implementation of Conway's Game of Life in Rust.

USAGE:
    cli-game-of-life [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <DELAY>    Sets the delay between game ticks. Value is in miliseconds [default: 500]
    -i, --input <INPUT>    Sets the input file to configure initial state of game

Have fun!
```
现在，可以写代码获取你输入的值:
```
if let Some(input) = matches.value_of("INPUT") {
    println!("A config file was passed: {}", input);
}
```
value_of() 将会返回  Option<T> 因此你可以输入的数据是否存在而采取相应的行动，我们把DELAY设置成了默认值，也就是说无论有没有解析到输入，我们都会可以采取行动.现在我们不会使用value_of(), 在这个项目里面，我们会使用flag接受参数。默认情况下，所有的clap参数都是flag，这也就是为什么我们在描述INPUT和DELAY的时候，需要添加take_values()方法。因为flag并不会有值，当我们使用它们，像在这种场景:
```
if matches.is_present("TEST") {
    println!("TEST!");
}
```
这里有太多的可能的配置，所以我只建议你用到配置的时候才去看[文档](https://docs.rs/clap/2.33.3/clap/struct.Arg.html)。

好的，我们通过配置，已经能让我们的应用接受参数了，但是他们不会做任何处理，接下来将会做一些处理。
## 控制速度
让我们使用DELAY参数，现在我们的游戏hard-code了500ms作为刷新下一个状态的频率，动态地改变它是很简单的，首先，我们需要去读并且解析(Duration::from_millis() accept u64)我们输入的参数:
```
let delay:u64 = matches.value_of("DELAY").unwrap().parse().unwrap();
```
我们的第一个unwrap(返回空，将会抛出panic)，来检查输入是否为空，第二个unwrap(如果返回Err，将会抛出panic)来检查输入是不是一个合法的int, panic时候，我们希望程序退出。如果你想定制第二个错误，你需要写下面的逻辑:
```
let delay: u64 = match matches.value_of("DELAY").unwrap().parse() {
    Ok(val) => val,
    Err(e) => {
        println!("Error parsing DELAY argument: {}", e);
        500
    }
};
```
然后我们可以吧poll 函数里面的500换成delay变量。如果你想测试脚本是否正确运行，你需要执行这样的脚本: ./cli-game-of-life -d 200(记住这个值是毫秒)
这里有个小问题。由于处理的方式，我们需要在delay ms后，才展示屏幕上面的内容，如果delay5秒，那么程序开始的5秒不会有任何输出。我们可以用"drawing"修复它，
代码:
```
loop {
    queue!(stdout, Clear(ClearType::All))?;
    let mut i = 0;
    while let Some(line) = game.row_as_string(i) {
        queue!(stdout, MoveTo(0, i as u16), Print(line))?;
        i += 1;
    }

    queue!(
        stdout,
        MoveTo(0, (i + 1) as u16),
        Print("Press Esc to exit...")
    )?;
    stdout.flush()?;
    if poll(Duration::from_millis(delay))? {
        if let Event::Key(KeyEvent { code, .. }) = read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    game.tick();
}
```
## 定义Universe
现在是使用INPUT参数的时候了，这个参数制定了universe的配置路径，文件将会是下面这种格式:
```
5
5
00000
00100
00010
01110
00000
```
第一行代表Universe的行数，第二行代表Universe的列数，接下来就是描述Universe每个格子的详情,0代表死,1代表或者。现在这里有两个地方你可以放置配置文件:

1. 项目的根目录，一些文件像是Cargo.toml就在这个里面，并且你能通过脚本cargo run -- -i INPUT跑你的应用。使用cargo运行之后的内容，都可以作为参数传递给你的项目。
2.  ./target/debug. 这意味着您需要在每次更改后重新构建，然后执行/debug/cli-game-of-life -i starship。
在本次教程里面，建议使用第一种方式，因为它更方便。上面的配置在“Game of Life”中称为starship pattern，因此我们将文件命名为一样的，然后继续下一步
我们将会读取这个文件，首先需要导入一个新的依赖:
```
use std::fs::File;
use std::io::{BufRead, BufReader};
```
下面是解析文件的函数，返回game::Universe:: 
```
fn create_game_from_file(path: &str) -> game::Universe {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut rows_number = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            rows_number = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Rows number not detected!");
        }
    };
    let mut cols_number = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            cols_number = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Columns number not detected!");
        }
    };
    let mut game_universe = game::Universe::new(cols_number, rows_number);
    let mut row = 0;
    let mut live_cells = Vec::<(u32, u32)>::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let mut col = 0;
                for char in line.chars() {
                    match char {
                        '1' => live_cells.push((row, col)),
                        _ => {}
                    }
                    col += 1;
                }
            }
            _ => break,
        }

        line.clear();
        row += 1;
    }
    game_universe.set_cells(&live_cells);
    game_universe
}
```
这看起来很长而且有一定重构的空间，但是比较容易理解:

1. 打开文件，写入BufReader。
2. 创建变量line读取每一行。
3. 尝试去解析行数和列数。
4. 创建新的 Universe。
5. 遍历剩余行，解析cell，写入vector。
6. 调用game_universe.set_cell方法，把vector的值写入对象，然后返回。
   
我们需要做的最后一件事情就是让我们的新的函数得到使用，在main函数里面删除初始化游戏的逻辑，并且把我们新的代码放在解析DELAY变量后面:
```
let mut game = match matches.value_of("INPUT") {
    Some(path) => create_game_from_file(path),
    None => {
        let mut default_game = game::Universe::new(5, 5);
        default_game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
        default_game
    }
};
```
这个逻辑很简单：我们尝试读取INPUT参数，如果一个通过了，我们接下来调用create_game_from_file方法，如果没通过，我们然后默认的universe。

现在我们可以调用cargo run -- -i starship并且享受美景!你可以使用更大的场地，类似15*15， 并且由于我们不校验参数，所以不需要在每行最后输入0。


## 总结

希望您喜欢本教程，多谢您的阅读！

---

译者介绍：

m1zzx2 ，Rust 初学者，知乎工程师。
