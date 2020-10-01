pub struct User {
    name: String,
}

pub struct Post {
    id: u32,
    title: String,
}

markup::define! {
    Page<'a>(user: &'a User, posts: &'a [Post]) {
        @markup::doctype()
        html {
            head {
                title { "Hello " @user.name }
            }
            body {
                #main.container {
                    @for post in *posts {
                        div#{format!("post-{}", post.id)}["data-id" = post.id] {
                            .title { @post.title }
                        }
                    }
                }
                @Footer { name: &user.name, year: 2020 }
            }
        }
    }

    Footer<'a>(name: &'a str, year: u32) {
        "(c) " @year " " @name
    }
}

fn main() {
    let user = User {
        name: "Ferris".into(),
    };

    let posts = [
        Post {
            id: 1,
            title: "Road to Rust 1.0".into(),
        },
        Post {
            id: 2,
            title: "Stability as a Deliverable".into(),
        },
        Post {
            id: 3,
            title: "Cargo: Rust's community crate host".into(),
        },
    ];

    println!(
        "{}",
        Page {
            user: &user,
            posts: &posts
        }
    )
}
