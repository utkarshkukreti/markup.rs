mod e0 {
    markup::define! {
        Hello {
            "Hello,"
            " "
            "world!\n"
            {1 + 2}
            {format!("{}{}", 3, 4)}
        }
    }
}

mod e1 {
    markup::define! {
        Hello {
            div
            br;
        }
    }
}

mod e2 {
    markup::define! {
        Hello {
            button#go.button."button-blue"
            button#"go-back".{1 + 2}.{2 + 3}
        }
    }
}

mod e3 {
    markup::define! {
        Hello {
            div[a = 1, b = "2", c? = true, d? = false]
            br[e = 3];
        }
    }
}

mod e4 {
    markup::define! {
        Hello {
            div[a = 1] {
                "One"
                {0 + 1}
            }
            div {
                "Two"
                {1 + 1}
            }
        }
    }
}

mod e5 {
    markup::define! {
        Hello {
            "<&\">"
            {markup::Raw("<span></span>")}
        }
    }
}

mod e6 {
    markup::define! {
        Hello(foo: u32, bar: u32, string: String) {
            div {
                {foo + bar}
                {string}
            }
        }
    }
}

mod e7 {
    markup::define! {
        Hello<'a, T: std::fmt::Debug, U>(arg: T, arg2: U, str: &'a str) where U: std::fmt::Display {
            div {
                {format!("{:?}", arg)}
                {format!("{}", arg2)}
                {str}
            }
        }
    }
}

mod e8 {
    markup::define! {
        Add(a: u32, b: u32) {
            span { {a + b} }
        }
        Hello {
            {Add { a: 1, b: 2 }}
            {Add { a: 3, b: 4 }}
        }
    }
}

mod e9 {
    markup::define! {
        Classify(value: i32) {
            {value}
            " is "
            @if *value < 0 {
                "negative"
            } else if *value == 0 {
                "zero"
            } else {
                "positive"
            }
            ".\n"
        }
        Main {
            {Classify { value: -42 }}
            " "
            {Classify { value: 0 }}
            " "
            {Classify { value: 42 }}
        }
    }
}

mod e10 {
    markup::define! {
        Main {
            @for i in 1..5 {
                {i} " * 2 = " {i * 2} ";\n"
            }
        }
    }
}

fn main() {
    println!("{}\n", e0::Hello {});
    println!("{}\n", e1::Hello {});
    println!("{}\n", e2::Hello {});
    println!("{}\n", e3::Hello {});
    println!("{}\n", e4::Hello {});
    println!("{}\n", e5::Hello {});
    println!(
        "{}\n",
        e6::Hello {
            foo: 1,
            bar: 2,
            string: String::from("hello")
        }
    );
    println!(
        "{}\n",
        e7::Hello {
            arg: (1, 2),
            arg2: "arg2",
            str: "str"
        }
    );
    println!("{}\n", e8::Hello {});
    println!("{}\n", e9::Main {});
    println!("{}\n", e10::Main {});
}
