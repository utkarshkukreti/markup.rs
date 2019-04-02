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
markup = "0.1"
```

Define your template using the `markup::define!` macro:

```rust
markup::define! {
    Hello(name: &'static str) {
        {markup::Doctype}
        html {
            head {
                title { "Hello " {name} }
            }
            body {
                p#greeting {
                    "Hello "
                    span.name { {name} }
                }
            }
        }
    }
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

The above template compiles to:

```rust
pub struct Hello {
    pub name: &'static str,
}

impl std::fmt::Display for Hello {
    fn fmt(&self, __writer: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Display;
        let Hello { name } = self;
        markup::Render::render(&(markup::Doctype), __writer)?;
        __writer.write_str("<html><head><title>Hello ")?;
        markup::Render::render(&(name), __writer)?;
        __writer.write_str("</title></head><body><p id=\"greeting\">Hello <span class=\"name\">")?;
        markup::Render::render(&(name), __writer)?;
        __writer.write_str("</span></p></body></html>")?;
        Ok(())
    }
}
```

Rendering the template produces (manually prettified):

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Hello Ferris</title>
  </head>
  <body>
    <p id="greeting">Hello <span class="name">Ferris</span></p>
  </body>
</html>
```

## Syntax

<!-- Syntax -->

### Define

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

### Literal Strings and Expressions

```rust
markup::define! {
    Hello {
        "Hello,"
        " "
        "world!\n"
        {1 + 2}
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
3345
```

### Elements

#### Normal and Void

```rust
markup::define! {
    Hello {
        div
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

#### id and class shorthands

```rust
markup::define! {
    Hello {
        button#go.button."button-blue"
        button#"go-back".{1 + 2}.{2 + 3}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<button id="go" class="button button-blue"></button><button id="go-back" class="3 5"></button>
```

#### Attributes with and without values

```rust
markup::define! {
    Hello {
        div[a = 1, b = "2", c? = true, d? = false, "e-f" = 3, {"g".to_string() + "-h"} = 4, i = None::<i32>, j = Some(5)]
        br[k = 6];
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<div a="1" b="2" c e-f="3" g-h="4" j="5"></div><br k="6">
```

#### Children

```rust
markup::define! {
    Hello {
        div[a = 1] {
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
<div a="1">One1</div><div>Two2</div>
```

### Disable Automatic HTML Escaping

```rust
markup::define! {
    Hello {
        "<&\">"
        {markup::Raw("<span></span>")}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
&lt;&amp;&quot;&gt;<span></span>
```

### Arguments

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
    Hello<'a, T: std::fmt::Debug, U>(arg: T, arg2: U, str: &'a str) where U: std::fmt::Display {
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

### Embed Other Templates

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

### If

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

### For

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

<!-- /Syntax -->
