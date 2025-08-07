# Mindustry Logic Interpreter
An interpreter that interprets Mindustry logic instructions. Keep in mind that this is in an alpha-esque state due to
it being highly incomplete. Only a few instructions are implemented (just enough for fibonacci) and function properly.

# Speed
As of now, this is shown to be capable of up to 20 million instructions per second. However, that can dwindle down to
around 10 million instructions per second, depending on how many instructions you use.

# Usage
Currently, this isn't designed to be used actively. This is closer to being a proof-of-concept rather than being a
functioning tool. If you do still wish to use this anyway, then you should be able to edit the fibonacci example and
run `cargo run` in the root directory of the project. This will open the fibonacci example file and run the interpreter.
*Note that the `set` instruction doesn't work properly. This is a bug, and is intended to be fixed eventually. The jump
instruction is also buggy, so do not expect it to work as intended. There are no labels.*

# Compilation
This can be compiled with Cargo. All you need to do is run `cargo build` --- or, if you wish to run it via cargo,
`cargo run`.
