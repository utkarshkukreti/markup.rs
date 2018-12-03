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

fn main() {
    println!("{}", Hello { name: "Ferris" });
}
