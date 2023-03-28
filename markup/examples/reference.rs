use std::io::Write;

fn _0() {
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
    writeln!(
        &mut std::io::stdout(),
        "{}",
        HelloGeneric { name: "World 2" }
    )
    .unwrap();

    // The template can also be rendered to a String:
    let string = Hello { name: "World 3" }.to_string();
    println!("{}", string);
}

fn _1() {
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
}

fn _2() {
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
}

fn _3() {
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
}

fn _4() {
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

    println!(
        "{}",
        Attributes {
            id: 123,
            category: String::from("tutorial")
        }
    );
}

fn _5() {
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
}

fn _6() {
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
}

fn _7() {
    markup::define! {
        For<'a>(xs: &'a [u32]) {
            @for (i, x) in xs.iter().enumerate() {
                @i ": " @x "\n"
            }
        }
    }

    println!("{}", For { xs: &[1, 2, 4, 8] });
}

fn main() {
    _0();
    println!("---");
    _1();
    println!("---");
    _2();
    println!("---");
    _3();
    println!("---");
    _4();
    println!("---");
    _5();
    println!("---");
    _6();
    println!("---");
    _7();
    println!("---");
}
