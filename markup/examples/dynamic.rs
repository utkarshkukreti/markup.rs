markup::define! {
    Tabs<'a>(tabs: &'a [Tab<'a>]) {
       @for tab in tabs.iter() {
            @tab
       }
    }

    Tab<'a>(
        title: &'a str,
        body: markup::DynRender<'a>,
    ) {
        h1 { @title }
        div { @body }
    }
}

fn main() {
    let tabs = [
        Tab {
            title: "Home",
            body: markup::new! {
                p { "This is the home page." }
            },
        },
        Tab {
            title: "About",
            body: markup::new! {
                p { "This is the about page." }
            },
        },
    ];

    println!("{}", Tabs { tabs: &tabs })
}
