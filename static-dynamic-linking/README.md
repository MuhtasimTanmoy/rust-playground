# Static Dynamic Linking

- `lib` 
    — Defines `execute` function

- `host` 
    — References `execute` in source
    - Links to `lib` statically
    - Provides the `execute` symbol in its executable binary

- `plugin` 
    — References `execute` in source
    - Does not link to `lib` at all
    - Leaving the `execute` symbol undefined at compile time. 
    - At runtime, `execute` will be accessible because `plugin` is loaded into `host`, gaining access to `host`'s symbols.



- Commands

If you need to have binutils first in your PATH, run:
  echo 'export PATH="/opt/homebrew/opt/binutils/bin:$PATH"' >> /Users/macbookpro2020/.zshrc

For compilers to find binutils you may need to set:
  export LDFLAGS="-L/opt/homebrew/opt/binutils/lib"
  export CPPFLAGS="-I/opt/homebrew/opt/binutils/include"

greadelf
gobjdump -p /path/to/app
greadelf -p .comment target/debug/echo
otool -l target/debug/echo
strings target/debug/echo | grep -i gcc
strings target/debug/echo | grep -i rustc
gobjdump -s -j __TEXT target/debug/echo
nm target/debug/echo

otool -l

objdump -j .comment -s

rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
dwarfdump target/debug/echo