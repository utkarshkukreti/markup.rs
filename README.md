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

The template can either be rendered to a String or written to a writer.

```
// Render to a String.
let string = Page { user: &user, posts: &posts }.to_string();

// Render and print directly to stdout.
println!(
    "{}",
    Page { user: &user, posts: &posts }
);

// Render and write to `writer`, which can be any value `std::write!` [1] supports.
// [1]: https://doc.rust-lang.org/stable/std/macro.write.html
write!(
    writer,
    "{}",
    Page { user: &user, posts: &posts }
);
```
