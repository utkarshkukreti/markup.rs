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
markup = "0.11.0"
```

## Quick Example

```rust
markup::define! {
    Home<'a>(title: &'a str) {
        @markup::doctype()
        html {
            head {
                title { @title }
                style {
                    "body { background: #fafbfc; }"
                    "#main { padding: 2rem; }"
                }
            }
            body {
                @Header { title }
                #main {
                    p {
                        "This domain is for use in illustrative examples in documents. You may \
                        use this domain in literature without prior coordination or asking for \
                        permission."
                    }
                    p {
                        a[href = "https://www.iana.org/domains/example"] {
                            "More information..."
                        }
                    }
                }
                @Footer { year: 2020 }
            }
        }
    }

    Header<'a>(title: &'a str) {
        header {
            h1 { @title }
        }
    }

    Footer(year: u32) {
        footer {
            "(c) " @year
        }
    }
}

fn main() {
    println!(
        "{}",
        Home {
            title: "Example Domain"
        }
    )
}
```

### Output (manually prettified)

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Example Domain</title>
    <style>
      body {
        background: #fafbfc;
      }
      #main {
        padding: 2rem;
      }
    </style>
  </head>
  <body>
    <header><h1>Example Domain</h1></header>
    <div id="main">
      <p>
        This domain is for use in illustrative examples in documents. You may
        use this domain in literature without prior coordination or asking for
        permission.
      </p>
      <p>
        <a href="https://www.iana.org/domains/example">More information...</a>
      </p>
    </div>
    <footer>(c) 2020</footer>
  </body>
</html>
```
