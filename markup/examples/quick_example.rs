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

    let name = "Ferris";
    let times = 5;
    println!(
        "{}",
        markup::render! {
            h1 { "Greeting" }
            @for _ in 0..times {
                p.greeting {
                    "Hello " {name} "!"
                }
            }
        }
    );
}
