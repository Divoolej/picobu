## PicoBu(ild) - the simple, one-command build system for PICO-8 games.
Picobu uses a sensible-defaults, zero-configuration approach. You can use it by simply typing in the `picobu` command in your project directory. It also supports a watch mode - rebuilding the cartridge on file changes.

### Usage
```bash
# Look for *.lua files in the src/ directory and compile them into a single *.p8 file.
$> picobu 

# Look for *.lua files in the code/ folder instead.
$> picobu -i code 

# Specify a name of the output file.
# Note that if a *.p8 file is already present in the current directory, picobu is smart enough to find it on it's own.
$> picobu pico_game.p8

# Enter the watch mode (recompile when source files change)
$> picobu -w
```
