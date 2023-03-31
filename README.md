<div align="center">

# markup.rs

A blazing fast, type-safe template engine for Rust.

[![Build](https://img.shields.io/github/actions/workflow/status/utkarshkukreti/markup.rs/build.yml?style=for-the-badge)](https://github.com/utkarshkukreti/markup.rs/actions/workflows/build.yml)
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
markup = "0.13.1"
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

### Output

```html
<!DOCTYPE html><html><head><title>Example Domain</title><style>body { background: #fafbfc; }#main { padding: 2rem; }</style></head><body><header><h1>Example Domain</h1></header><div id="main"><p>This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.</p><p><a href="https://www.iana.org/domains/example">More information...</a></p></div><footer>(c) 2020</footer></body></html>
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

## Syntax Reference (WIP)

### markup::define! and markup::new!

There are two ways to define templates: `markup::define!` and `markup::new!`.

`markup::define!` defines a template with named arguments. These templates cannot access variables from outer scope. The templates can have generic parameters. Under the hood, `markup::define!` compiles to a Rust struct that implements `markup::Render` and `std::fmt::Display` traits.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Hello<'a>(name: &'a str) {
          "Hello, " @name "!"
      }
      HelloGeneric<T: std::fmt::Display>(name: T) {
          "Hello, " @name.to_string() "!"
      }
  }

  // The template can now be printed directly or written to a stream:
  println!("{}", Hello { name: "World" });
  writeln!(&mut std::io::stdout(), "{}", HelloGeneric { name: "World 2" }).unwrap();

  // The template can also be rendered to a String:
  let string = Hello { name: "World 3" }.to_string();
  println!("{}", string);
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  Hello, World!
  Hello, World 2!
  Hello, World 3!
  ```
  </td></tr>
</table>


`markup::new!` defines a template without any arguments. These can access variables from outer scope.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  let name = "World";
  let template = markup::new! {
      "Hello, " @name "!"
  };

  // The template can now be printed directly or written to a stream:
  println!("{}", template);
  writeln!(&mut std::io::stdout(), "{}", template).unwrap();

  // The template can also be rendered to a String:
  let string = template.to_string();
  println!("{}", string);
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  Hello, World!
  Hello, World!
  Hello, World!
  ```
  </td></tr>
</table>


### Expressions

Templates can have bare literal values, which are rendered as is. They can also have expressions (including function and macro calls) preceded by `@` sign. All strings are HTML-escaped unless they are wrapped in `markup::raw()`.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Expressions(a: i32, b: i32) {
          1 " + " 2 " = " @{1 + 2} '\n'
          @a " - " @b " = " @{a - b} '\n'
          @format!("{} * {} = {}", a, b, a * b) '\n'
          @a " ^ 4 = " @a.pow(4) '\n'

          // All output is escaped by default.
          "<>\n"
          // Escaping can be disabled using `markup::raw()`.
          @markup::raw("<div></div>")
      }
  }

  println!("{}", Expressions { a: 5, b: 3 });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  1 + 2 = 3
  5 - 3 = 2
  5 * 3 = 15
  5 ^ 4 = 625
  &lt;&gt;
  <div></div>
  ```
  </td></tr>
</table>


### Elements

Elements are defined using a CSS selector-like syntax. Elements can contain other nested elements in braces or be followed by a semicolon for self-closing elements.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Elements(name: &'static str) {
          // Just a div.
          div {}
          '\n'
          // Three nested elements.
          main {
              aside {
                  h3 { "Sidebar" }
              }
          }
          '\n'
          // Self-closing input element.
          input;
          '\n'
          // Element with a name containing dashes.
          $"my-custom-element" {}
          '\n'
          // Element with a dynamic name.
          ${name} {}
      }
  }

  println!("{}", Elements { name: "span" });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  <div></div>
  <main><aside><h3>Sidebar</h3></aside></main>
  <input>
  <my-custom-element></my-custom-element>
  <span></span>
  ```
  </td></tr>
</table>


### Attributes

Attributes are defined after the element name. `id` and `class` attributes can be defined using CSS selector-like syntax using `#` and `.`. Classes may be specified multiple times using this shorthand syntax. Other attributes are specified in square brackets.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Attributes(id: u32, category: String, data: std::collections::BTreeMap<String, String>) {
          // A div with an id and two classes.
          div#foo.bar.baz {}
          '\n'
          // A div with a dynamically computed id and one static and one dynamic class.
          div#{format!("post-{}", id)}.post.{format!("category-{}", category)} {}
          '\n'

          // Boolean attributes are only rendered if true. Specifying no value is the same as `true`.
          input[checked = true];
          '\n'
          input[checked = false];
          '\n'
          input[checked];
          '\n'

          // `Option` attributes are rendered only if they're `Some`.
          input[type = Some("text"), minlength = None::<String>];
          '\n'

          // Attribute names can also be expressions wrapped in braces.
          div[{format!("{}{}", "data-", "post-id")} = id] {}
          '\n'

          // Multiple attributes can be added dynamically using the `..` syntax.
          div[..data.iter().map(|(k, v)| (("data-", k), v))] {}
      }
  }

  println!("{}", Attributes {
      id: 123,
      category: String::from("tutorial"),
      data: [
          (String::from("foo"), String::from("bar")),
          (String::from("baz"), String::from("quux"))
      ].iter().cloned().collect(),
  });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  <div id="foo" class="bar baz"></div>
  <div id="post-123" class="post category-tutorial"></div>
  <input checked>
  <input>
  <input checked>
  <input type="text">
  <div data-post-id="123"></div>
  <div data-baz="quux" data-foo="bar"></div>
  ```
  </td></tr>
</table>


### @if and @if let

`@if` and `@if let` works similar to Rust.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      If(x: u32, y: Option<u32>) {
          @if *x == 1 {
              "x = 1\n"
          } else if *x == 2 {
              "x = 2\n"
          } else {
              "x is neither 1 nor 2\n"
          }

          @if let Some(y) = y {
              "y = " @y "\n"
          } else {
              "y is None\n"
          }
      }
  }

  println!("{}", If { x: 2, y: Some(2) });
  println!("{}", If { x: 3, y: None });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  x = 2
  y = 2

  x is neither 1 nor 2
  y is None
  ```
  </td></tr>
</table>


### @match

`@match` works similar to Rust, but the branches must be wrapped in braces.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Match(x: Option<u32>) {
          @match x {
              Some(1) | Some(2) => {
                  "x is 1 or 2"
              }
              Some(x) if *x == 3 => {
                  "x is 3"
              }
              None => {
                  "x is None"
              }
              _ => {
                  "x is something else"
              }
          }
      }
  }

  println!("{}", Match { x: None });
  println!("{}", Match { x: Some(2) });
  println!("{}", Match { x: Some(4) });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  x is None
  x is 1 or 2
  x is something else
  ```
  </td></tr>
</table>


### @for

`@for` works similar to Rust.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      For<'a>(xs: &'a [u32]) {
          @for (i, x) in xs.iter().enumerate() {
              @i ": " @x "\n"
          }
      }
  }

  println!("{}", For { xs: &[1, 2, 4, 8] });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  0: 1
  1: 2
  2: 4
  3: 8
  ```
  </td></tr>
</table>


### Statements

Templates can have statements preceded by `@` sign. The most useful such
statement is `@let` to compute a value for later reuse. `@fn` can be used to
define a function. Also supported are `@struct`, `@mod`, `@impl`, `@const`,
`@static` and more.

<table>
  <tr><th>Code</th></tr>
  <tr><td width="1000px">

  ```rust
  markup::define! {
      Statement(x: i32) {
          @let double_x = x * 2;
          @double_x '\n'

          @fn triple(x: i32) -> i32 { x * 3 }
          @triple(*x)
      }
  }

  println!("{}", Statement { x: 2 });
  ```
  </td></tr>
  <tr><th>Output</th></tr>
  <tr><td width="1000px">

  ```html
  4
  6
  ```
  </td></tr>
</table>
