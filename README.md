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
markup = "0.9.1"
```

### Template

```rust
pub struct User {
    name: String,
}

pub struct Post {
    id: u32,
    title: String,
}

markup::define! {
    Page<'a>(user: &'a User, posts: &'a [Post]) {
        @markup::doctype()
        html {
            head {
                title { "Hello " @user.name }
            }
            body {
                #main.container {
                    @for post in *posts {
                        div#{format!("post-{}", post.id)}["data-id" = post.id] {
                            .title { @post.title }
                        }
                    }
                }
                @Footer { name: &user.name, year: 2020 }
            }
        }
    }

    Footer<'a>(name: &'a str, year: u32) {
        "(c) " @year " " @name
    }
}

fn main() {
    let user = User {
        name: "Ferris".into(),
    };

    let posts = [
        Post {
            id: 1,
            title: "Road to Rust 1.0".into(),
        },
        Post {
            id: 2,
            title: "Stability as a Deliverable".into(),
        },
        Post {
            id: 3,
            title: "Cargo: Rust's community crate host".into(),
        },
    ];

    println!(
        "{}",
        Page {
            user: &user,
            posts: &posts
        }
    )
}

```

### Output (manually prettified)

```
<!DOCTYPE html>
<html>
  <head>
    <title>Hello Ferris</title>
  </head>
  <body>
    <div id="main" class="container">
      <div id="post-1" data-id="1">
        <div class="title">Road to Rust 1.0</div>
      </div>
      <div id="post-2" data-id="2">
        <div class="title">Stability as a Deliverable</div>
      </div>
      <div id="post-3" data-id="3">
        <div class="title">Cargo: Rust's community crate host</div>
      </div>
    </div>
    (c) 2020 Ferris
  </body>
</html>
```

The template can either be rendered as a String or streamed into a writer.

```
let into_string = Page { user: &user, posts: &posts }.to_string();

println!(
    "{}",
    Page { user: &user, posts: &posts }
)

write!(
    writer,
    "{}",
    Page { user: &user, posts: &posts }
)
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

A template can have bare literal values and arbitrary expressions in braces.

```rust
markup::define! {
    Hello {
        "Hello,"
        " "
        "world!\n"
        @{1 + 2}
        @{'π'}
        @{format!("{}{}", 3, 4)}
        @{if true { Some(5) } else { None }}
        @{if false { Some(6) } else { None }}
        @7
        @{8 + 9}
        10
        11.12
        {13 + 14}
    }
}
```
```rust
println!("{}", Hello {});
```
```html
Hello, world!
3π3457171011.1227
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
            @{0 + 1}
        }
        div {
            "Two"
            @{1 + 1}
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
        @markup::raw("<span></span>")
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
            @{foo + bar}
            @string
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
    Hello<'a, T: std::fmt::Debug, U, V: markup::Render>(
        arg: T,
        arg2: U,
        str: &'a str,
        v: V,
    ) where U: std::fmt::Display {
        div {
            @format!("{:?}", arg)
            @format!("{}", arg2)
            @str
            @v
        }
    }
}
```
```rust
println!("{}", Hello { arg: (1, 2), arg2: "arg2", str: "str", v: markup::new!(foo {}) });
```
```html
<div>(1, 2)arg2str<foo></foo></div>
```

Other templates can be embedded by simply putting them in braces.

```rust
markup::define! {
    Add(a: u32, b: u32) {
        span { @{a + b} }
    }
    Hello {
        @Add { a: 1, b: 2 }
        @Add { a: 3, b: 4 }
    }
}
```
```rust
println!("{}", Hello {});
```
```html
<span>3</span><span>7</span>
```

`@` can be followed by any [Item](https://docs.rs/syn/1/syn/enum.Item.html):

```rust
markup::define! {
    Main {
        @const ONE: i32 = 1;
        @#[derive(Debug)] struct Int(i32);
        @let Int(two) = Int(2);
        @format!("{} {}\n", ONE, two)
        @two.pow(10) "\n"
        @mod three {
            pub const THREE: i32 = 3;
        }
        @fn four() -> i32 {
            4
        }
        @for x in &[three::THREE, four(), 5] {
            @let (_double, triple) = (x * 2, x * 3);
            @x " * 3 = " @triple "\n"
        }
    }
}
```
```rust
println!("{}", Main {});
```
```html
1 2
1024
3 * 3 = 9
4 * 3 = 12
5 * 3 = 15
```

@if

```rust
markup::define! {
    Classify(value: i32) {
        @value
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
        @Classify { value: -42 }
        @Classify { value: 0 }
        @Classify { value: 42 }
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
        @if let Some(0) = *value {
            "Some(ZERO)"
        } else if let Some(value) = *(value) {
            "Some(" @value ")"
        } else {
            "None"
        }
        "\n"
    }
    Main {
        @Classify { value: None }
        @Classify { value: Some(0) }
        @Classify { value: Some(1) }
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
        @match *value {
          Some(1) | Some(2) => {
            "1"
            " or 2"
          }
          Some(n) if n == 3 => {
            @n @n
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
        @Classify { value: None }
        @Classify { value: Some(0) }
        @Classify { value: Some(1) }
        @Classify { value: Some(2) }
        @Classify { value: Some(3) }
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
            @i " * 2 = " @{i * 2} ";\n"
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
