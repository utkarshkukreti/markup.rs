markup::define! {
    Hello<'a>(name: &'a str) {
        {markup::Doctype}
        html {
            head {
                title { "Hello " {name} }
            }
            body {
                div#main.container {
                    {Greeting { name: "Everyone!" }}
                    br;
                    {Greeting { name: name }}
                }
            }
        }
    }
    Greeting<'a>(name: &'a str) {
        p.greeting.{if *name == "Ferris" { Some("ferris" ) } else { None }} {
            "Hello "
            span.name {
                @if *name == "Ferris" {
                    "FERRIS"
                    @for _ in 0..3 {
                        "!"
                    }
                } else {
                    {name}
                }
            }
        }
    }
}

fn main() {
    println!("{}", Hello { name: "Ferris" });
}
