# DodgeR (gdext evaluation project)

This is part of a project to evaluate the godot-rust bindings [gdext](https://github.com/godot-rust/gdext).

It is derived from the Dodge-the-creeps example game: 
[gdext dodge-the-creeps](https://github.com/godot-rust/gdext/tree/master/examples/dodge-the-creeps),
which is a finished Rust version of the game featured in the
["Your first 2D game"](https://docs.godotengine.org/en/latest/getting_started/first_2d_game/index.html)
tutorial in the godot documentation.

Language: Rust

Renderer: Vulkan Mobile

The evaluation project consists of four Repositories:
- [overview repository](https://github.com/franziskusz/gdext-evaluation) (contains raw data, results and a description of the evaluation process)
- [this](https://github.com/franziskusz/dodge-r)
- [GDScript Version](https://github.com/franziskusz/dodge-gds)
- [process-logger](https://github.com/franziskusz/process-logger)
- [python-pandas-plotter](https://github.com/franziskusz/pandas-plotter)

## Setup
If the [prebuild executables](https://github.com/franziskusz/dodge-r/releases/tag/v0.1.0-alpha) [should not work for you](https://github.com/franziskusz/dodge-r/issues/1), you can build the project from source with the following steps:
1. [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. [Download Godot 4.2](https://godotengine.org/download/archive/4.2-stable/)
3. [Setup gdext](https://godot-rust.github.io/book/intro/setup.html) by making the Godot executable available. Either way by:
   - putting it in your `PATH` as `godot4`
   - or as an environment variable called `GODOT4_BIN`, containing the path to the Godot executable.
4. Clone this repository by opening a shell and entering:
   - `git clone git@github.com:franziskusz/dodge-r.git` (requires having a ssh key setup)
   - or `git clone https://github.com/franziskusz/dodge-r.git` 
5. Change to the just cloned repository directory with `cd dodge-r` (Unix)
6. Run `cargo build` and `cargo build --release` (this project is configured for performance. A full release build is expected to take over 10 minutes)
7. Open the `dodge-the-creeps/godot/project.godot` file by double clicking it.
8. With entering `command b` or clicking the triangle on the top right you can run the project within the editor in debug mode
9. In the menu select `Project -> Export...` to export the release build
10. Select `Add..` at the top/center to add a `Preset` for your Operating System (This might require an additional autmatic download)
11. Select `Export Project` at the bottom and chose a name and target directory, deselect `Export With Debug` and click on `Save`

## Notes
- To dodge breaking changes of the gdext bindings I included a local version within this repository (`/gdext/`). It is an unmodified Version from early January 2024.
- I am new to Godot and Rust. This whole project is also a learning experience for me. If the way some things are implemented gives you headaches, I apologize. I am open for any kind of criticism.
- .csv logging is based on the Rust crate [csv](https://crates.io/crates/csv)
- The additional calculation workload is based on [godot-rust-benchmark](https://github.com/extrawurst/godot-rust-benchmark/tree/main)
- This applications writes performance logs to timestamped .csv files within the `/app_userdata/DodgeR/stats/` directory. See [godot file paths](https://docs.godotengine.org/en/stable/tutorials/io/data_paths.html) for details. ***This directory will never get cleared automatically!***



## Screenshots

![main_menu](dodge-the-creeps/godot/screenshots/dodger_main.jpg)
![run](dodge-the-creeps/godot/screenshots/dodger_run.jpg)

## Copying
`dodge-the-creeps/godot/logo.png`, `dodge-the-creeps/godot/art/background`, `dodge-the-creeps/godot/art/crab`, `dodge-the-creeps/godot/art/creep1`, `dodge-the-creeps/godot/art/creep2`, `dodge-the-creeps/godot/art/creep3` Copyright &copy; 2024 [franziskusz](https://github.com/franziskusz), [CC BY-NC 4.0: Attribution-NonCommercial](https://creativecommons.org/licenses/by-nc/4.0/)

`dodge-the-creeps/godot/art/House In a Forest Loop.ogg` Copyright &copy; 2012 [HorrorPen](https://opengameart.org/users/horrorpen), [CC-BY 3.0: Attribution](http://creativecommons.org/licenses/by/3.0/). Source: https://opengameart.org/content/loop-house-in-a-forest

Font is "Xolonium". Copyright &copy; 2011-2016 Severin Meyer <sev.ch@web.de>, with Reserved Font Name Xolonium, SIL open font license version 1.1. Details are in `dodge-the-creeps/godot/fonts/LICENSE.txt`.
