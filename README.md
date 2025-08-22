# Mindustry Logic Interpreter
An interpreter that interprets Mindustry logic instructions. This is intended to trade off the behavior accuracy
relative to mlog in order to maximize the instruction speed. This will also assume that the developer knows what they
are doing.

# Similar and equally-cool projects
## [Mindy](https://mindy.object.gay/)
This project, which is also written in Rust, is designed to emulate the entire mlog experience. If you're looking for
something to emulate mlog in without having to use Mindustry itself — then I'd 100% recommend this project.

## [Mily VM](https://github.com/ElectricGun/mily-vm)
This is similar in concept to my project. I'm not 100% sure of what the exact goals are, but the developer designed this
to act as an emulator for the mlog that his compiler creates. The developer behind this project is passionate, and a
close friend of mine, so I'd recommend this project over mine if you aren't seeking a high-speed low-cost mlog
interpreter.

# Speed
**Up to** 232 million instructions/second with just noops. I plan on hopefully increasing this.

# Usage
Currently, this isn't designed to be used actively. This is closer to being a proof-of-concept rather than being a
functioning tool. If you do still wish to use this anyway, then you should be able to edit the fibonacci example and
run `cargo run` in the root directory of the project. This will open the fibonacci example file and run the interpreter.
*Note that the `set` instruction doesn't work properly. This is a bug, and is intended to be fixed eventually. The jump
instruction is also buggy, so do not expect it to work as intended. There are no labels.*

# Compilation
This can be compiled with Cargo. All you need to do is run `cargo build` --- or, if you wish to run it via cargo,
`cargo run`.
