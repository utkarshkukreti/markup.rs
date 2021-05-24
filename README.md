<div align="center">

# markup.rs

A blazing fast, type-safe template engine for Rust.

[![Build](https://img.shields.io/github/workflow/status/utkarshkukreti/markup.rs/Build?style=for-the-badge)](https://github.com/utkarshkukreti/markup.rs/actions/workflows/build.yml)
[![Version](https://img.shields.io/crates/v/markup?style=for-the-badge)](https://crates.io/crates/markup)
[![Documentation](https://img.shields.io/docsrs/markup?style=for-the-badge)](https://docs.rs/markup)
[![Downloads](https://img.shields.io/crates/d/markup?style=for-the-badge)](https://crates.io/crates/markup)
[![License](https://img.shields.io/crates/l/markup?style=for-the-badge)](https://crates.io/crates/markup)

</div>

`markup.rs` is a template engine for Rust powered by procedural macros which
parses the template at compile time and generates optimal Rust code to render
the template at run time. The templates may embed Rust code which is type
checked by the Rust compiler enabling full type-safety.

## Features

* Fully type-safe with inline highlighted errors when using editor extensions like [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer).
* Less error-prone and terse syntax inspired by [Haml](https://haml.info/), [Slim](http://slim-lang.com/), and [Pug](https://pugjs.org).
* Zero unsafe code.
* Zero runtime dependencies.
* ⚡ Blazing fast. The fastest in [this](https://github.com/djc/template-benchmarks-rs) benchmark among the ones which do not use unsafe code, the second fastest overall.

## Install

```toml
[dependencies]
markup = "0.12.2"
```

## Example

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
