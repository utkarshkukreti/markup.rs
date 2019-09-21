# markup.rs

> A blazing fast, type-safe template engine for Rust.

`markup.rs` is a template engine for Rust powered by procedural macros which
parses the template at compile time and generates optimal Rust code to render
the template at run time. The templates may embed Rust code which is type
checked by the Rust compiler enabling full type-safety.

## Quick Start

Add the `markup` crate to your dependencies:

```toml
[dependencies]
markup = "0.4.1"
```

Define your template using the `markup::define!` macro:

```rust
markup::define! {
    Hello<'a>(name: &'a str) {
        {markup::doctype()}
        html {
            head {
                title { "Hello " {name} }
            }
            body {
                #main.container {
                    {Greeting { name: "Everyone!" }}
                    br;
                    {Greeting { name: name }}
                }
            }
        }
    }
    Greeting<'a>(name: &'a str) {
        p.greeting {
            "Hello " {name} "!"
        }
    }
}

fn main() {
    println!("{}", Hello { name: "Ferris" });
}
```

Render your template by either:

1. Writing it to any instance of `std::io::Write`:

   ```rust
   write!(writer, "{}", Hello { name: "Ferris" });
   ```

2. Converting it to a string and using it however you like:

   ```rust
   let string = Hello { name: "Ferris" }.to_string();
   ```

Rendering the template produces (manually prettified):

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Hello Ferris</title>
  </head>
  <body>
    <div id="main" class="container">
      <p class="greeting">Hello Everyone!</p>
      <br>
      <p class="greeting">Hello Ferris!</p>
    </div>
  </body>
</html>
```

## Note

Due to [a limitation in syn](https://github.com/dtolnay/syn/issues/515), the
crate this crate uses to parse templates, it is necessary to wrap identifiers
which precede an opening brace to be put in parentheses.

Example:

```rust
// Wrong
@if let Some(foo) = bar {
  ...
}
@match foo {
  ...
}

// Correct
@if let Some(foo) = {bar} {
  ...
}
@match {foo} {
  ...
}

// Also works
@if let Some(foo) = *&(bar) {
  ...
}
@match *&(foo) {
  ...
}
```

## Syntax

<!-- Syntax -->

You can define multiple templates in a `define!` block.

```rust
markup::define! {
    First {
      "First!"
    }
    Second {
      "Second!"
    }
}
```

```rust
println!("{}", First);
println!("{}", Second.to_string());
```

```html
First!
Second!
```

A template can have bare literal strings and arbitrary expressions in braces.

```rust
markup::define! {
    Hello {
        "Hello,"
        " "
        "world!\n"
        {1 + 2}
        {'π'}
        {format!("{}{}", 3, 4)}
        {if true { Some(5) } else { None }}
        {if false { Some(6) } else { None }}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
Hello, world!
3π345
```

Elements can either have children inside `{}` or be a void tag, ending with `;`.

```rust
markup::define! {
    Hello {
        div {}
        br;
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<div></div><br>
```

An id and multiple classes can be applied to an element using CSS like selectors.
The value after `#` and `.` can be either an identifier, a literal string, or an expression inside braces.

```rust
markup::define! {
    Hello {
        .foo {
            .bar {}
        }
        button#go.button."button-blue" {}
        button#"go-back".{1 + 2}.{2 + 3} {}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<div class="foo"><div class="bar"></div></div><button id="go" class="button button-blue"></button><button id="go-back" class="3 5"></button>
```

Attributes can either be normal or boolean (ends with ?).
The name can be an identifier, a literal string, or an expression inside braces.
Boolean attributes are printed without value if true and omitted if false.
The value can be an Option, where None values are omitted and Some are unwrapped.

```rust
markup::define! {
    Hello {
        div[
            a = 1,
            b = "2",
            c? = true,
            d? = false,
            "e-f" = 3,
            {"g".to_string() + "-h"} = 4,
            i = None::<i32>,
            j = Some(5)
        ] {}
        "\n"
        br[k = 6];
        "\n"
        input[type = "text"];
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<div a="1" b="2" c e-f="3" g-h="4" j="5"></div>
<br k="6">
<input type="text">
```

An element can have zero or more children inside braces.

```rust
markup::define! {
    Hello {
        .foo[a = 1] {
            "One"
            {0 + 1}
        }
        div {
            "Two"
            {1 + 1}
        }
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<div class="foo" a="1">One1</div><div>Two2</div>
```

Automatic HTML escaping can be disabled using the `markup::raw` function.
This function accepts any type implementing std::fmt::Display.

```rust
markup::define! {
    Hello {
        "<&\">"
        {markup::raw("<span></span>")}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
&lt;&amp;&quot;&gt;<span></span>
```

A template can accept simple arguments as well as generic arguments with where clauses.
The arguments may have an optional trailing comma.

```rust
markup::define! {
    Hello(foo: u32, bar: u32, string: String) {
        div {
            {foo + bar}
            {string}
        }
    }
}
```
```rust
println!("{}", Hello { foo: 1, bar: 2, string: String::from("hello") });
```
```html
<div>3hello</div>
```

```rust
markup::define! {
    Hello<'a, T: std::fmt::Debug, U>(
        arg: T,
        arg2: U,
        str: &'a str,
    ) where U: std::fmt::Display {
        div {
            {format!("{:?}", arg)}
            {format!("{}", arg2)}
            {str}
        }
    }
}
```
```rust
println!("{}", Hello { arg: (1, 2), arg2: "arg2", str: "str" });
```
```html
<div>(1, 2)arg2str</div>
```

Other templates can be embedded by simply putting them in braces.

```rust
markup::define! {
    Add(a: u32, b: u32) {
        span { {a + b} }
    }
    Hello {
        {Add { a: 1, b: 2 }}
        {Add { a: 3, b: 4 }}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<span>3</span><span>7</span>
```

@if

```rust
markup::define! {
    Classify(value: i32) {
        {value}
        " is "
        @if *value < 0 {
            "negative"
        } else if *value == 0 {
            "zero"
        } else {
            "positive"
        }
        ".\n"
    }
    Main {
        {Classify { value: -42 }}
        " "
        {Classify { value: 0 }}
        " "
        {Classify { value: 42 }}
    }
}
```
```rust
println!("{}", Main {});
```
```html
-42 is negative.
 0 is zero.
 42 is positive.
```

@if let

```rust
markup::define! {
    Classify(value: Option<i32>) {
        @if let Some(0) = *(value) {
            "Some(ZERO)"
        } else if let Some(value) = *(value) {
            "Some(" {value} ")"
        } else {
            "None"
        }
        "\n"
    }
    Main {
        {Classify { value: None }}
        {Classify { value: Some(0) }}
        {Classify { value: Some(1) }}
    }
}
```
```rust
println!("{}", Main {});
```
```html
None
Some(ZERO)
Some(1)
```

@match

```rust
markup::define! {
    Classify(value: Option<i32>) {
        @match *(value) {
          Some(1) | Some(2) => {
            "1"
            " or 2"
          }
          Some(n) if n == 3 => {
            {n} {n}
          }
          Some(_) => {
            "Other"
          }
          None => {
            "None"
          }
        }
        "\n"
    }
    Main {
        {Classify { value: None }}
        {Classify { value: Some(0) }}
        {Classify { value: Some(1) }}
        {Classify { value: Some(2) }}
        {Classify { value: Some(3) }}
    }
}
```
```rust
println!("{}", Main {});
```
```html
None
Other
1 or 2
1 or 2
33
```

@for

```rust
markup::define! {
    Main {
        @for i in 1..5 {
            {i} " * 2 = " {i * 2} ";\n"
        }
    }
}
```
```rust
println!("{}", Main {});
```
```html
1 * 2 = 2;
2 * 2 = 4;
3 * 2 = 6;
4 * 2 = 8;
```

Curly braces also accept single statements and items and outputs it as-is in the generated code.

```rust
markup::define! {
    Main {
        {let x = 1;}
        {fn add1(x: i32) -> i32 {
            x + 1
        }}
        {add1(x)}
    }
}
```
```rust
println!("{}", Main {});
```
```html
2
```

<!-- /Syntax -->
