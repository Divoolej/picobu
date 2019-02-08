## PicoBu(ild) - the simple, one-command build system for PICO-8 games. Written in Rust :crab:
Picobu uses a sensible-defaults, zero-configuration approach. You can use it by simply typing in the `picobu` command in your project directory. It also supports a watch mode - rebuilding the cartridge on file changes.

### Usage
```fish
# Look for *.lua files in the src/ directory and compile them into a single *.p8 file.
❯ picobu

# Look for *.lua files in the code/ folder instead.
❯ picobu -i code

# Specify a name of the output file.
# Note that if a *.p8 file is already present in the current directory, picobu is smart enough to find it on it's own.
❯ picobu pico_game.p8

# Enter the watch mode (recompile when source files change)
❯ picobu -w
```

### Installation

#### Cross-platform
<<<<<<< Updated upstream
The easiest way to install Picobu on any OS is by using [cargo](https://www.rust-lang.org/tools/install):
```fish
❯ cargo install picobu
```

#### Mac OS
Get it from Homebrew:
```fish
❯ brew install divoolej/tap/picobu
```

#### Windows
If you don't want to install cargo, you can download the latest release [here](https://github.com/Divoolej/picobu/releases/latest).
Keep in mind you'll have to add the executable to you PATH manually.

#### Linux
I don't provide pre-built Linux binaries at the moment, so you'll have to use cargo.

### Building from source

Building Picobu is very easy. Clone the repository, make sure you have the latest version of Rust installed (I use stable) and simply run `cargo build --release`

### Contributing

* Fork it (https://github.com/divoolej/picobu/fork)
* Create your feature branch (git checkout -b my-new-feature)
* Commit your changes (git commit -am 'Add some feature')
* Push to the branch (git push origin my-new-feature)
* Create a new Pull Request
