markup::define! {
    Hello<'a>(name: &'a str) {
        @markup::doctype()
        html {
            head {
                title { "Hello " @name }
            }
            body {
                #main.container {
                    @Greeting { name: "Everyone!" }
                    br;
                    @Greeting { name: name }
                }
            }
        }
    }

    #[derive(Clone)]
    /// A doc comment.
    Greeting<'a>(name: &'a str) {
        p.greeting {
            "Hello " @name "!"
        }
    }
}

fn main() {
    println!("{}", Hello { name: "Ferris" });

    let name = "Ferris";
    let times = 5;

    println!(
        "{}",
        markup::to_string! {
            h1 { "Greeting" }
            @for _ in 0..times {
                "Hello " @name "!"
            }
        }
    );

    let mut string = String::new();
    markup::to_writer! (
        &mut string =>
        h1 { "Greeting" }
        @for _ in 0..times {
            "Hello " @name "!"
        }
    )
    .unwrap();
    println!("{}", string);
}
