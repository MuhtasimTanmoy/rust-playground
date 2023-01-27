## Lifetime Operator

```rust

// params with two lifetime as generic. Return depends only on first param's lifetime.

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
}


// Perfectly handles this case

fn skip_prefix(line: &str, prefix: &str) -> &str {
    // ...
}

let line = "lang:en=Hello World!";
let lang = "en";

let v;
{
    let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
    v = skip_prefix(line, p.as_str());  //  |
}                                       // -+ `p` goes out of scope.
println!("{}", v);


// struct usage

// To ensure that any reference to a Foo cannot outlive the reference to an i32 it contains.

struct Foo<'a> {
    x: &'a i32,
}


// Named lifetimes are a way of giving these scopes a name. Giving something a name is the first step towards being able to talk about it.


// static lifetime

let x: &'static str = "Hello, world.";

static FOO: i32 = 5;
let x: &'static i32 = &FOO;

```

## Lifetime elision

Here explicit lifetime declation is elicited. There are the three rules:

- Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.

- If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.

- If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.