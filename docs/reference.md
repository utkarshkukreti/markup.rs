# markup::define! and markup::new!

There are two ways to define templates: `markup::define!` and `markup::new!`.

`markup::define!` defines a template with named arguments. These templates cannot access variables from outer scope. The templates can have generic parameters. Under the hood, `markup::define!` compiles to a Rust struct that implements `markup::Render` and `std::fmt::Display` traits.

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

`markup::new!` defines a template without any arguments. These can access variables from outer scope.

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

# Expressions

Templates can have bare literal values, which are rendered as is. They can also have expressions (including function and macro calls) preceded by `@` sign. All strings are HTML-escaped unless they are wrapped in `markup::raw()`.

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

# Elements

Elements are defined using a CSS selector-like syntax. Elements can contain other nested elements in braces or be followed by a semicolon for self-closing elements.

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

# Attributes

Attributes are defined after the element name. `id` and `class` attributes can be defined using CSS selector-like syntax using `#` and `.`. Classes may be specified multiple times using this shorthand syntax. Other attributes are specified in square brackets.

```rust
markup::define! {
    Attributes(id: u32, category: String) {
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
    }
}

println!("{}", Attributes { id: 123, category: String::from("tutorial") });
```

# @if and @if let

# @match

# @for
