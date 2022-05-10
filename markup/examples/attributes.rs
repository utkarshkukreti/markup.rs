markup::define! {
    #[derive(Clone)]
    Page<Body: markup::Render>(
        /// The page title
        title: &'static str,
        /// The page body
        body: Body,
    ) {
        html {
            head {
                title { @title }
            }
            body {
                @body
            }
        }
    }
}

fn main() {
    let page = Page {
        title: "Hello",
        body: markup::new! {
            p {
                "Hello!"
            }
        },
    };
    println!("{}", page);
}
