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