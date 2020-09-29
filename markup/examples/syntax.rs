mod e0 {
    markup::define! {
        Hello {
            "Hello,"
            " "
            "world!\n"
            {1 + 2}
            {'π'}
            {format!("{}{}", 3, 4)}
            {if true { Some(5) } else { None }}
            {if false { Some(6) } else { None }}
            @7
            @{8 + 9}
        }
    }
}

mod e1 {
    markup::define! {
        Hello {
            div {}
            br;
        }
    }
}

mod e2 {
    markup::define! {
        Hello {
            .foo {
                .bar {}
            }
            button#go.button."button-blue" {}
            button#"go-back".{1 + 2}.{2 + 3} {}
        }
    }
}

mod e3 {
    markup::define! {
        Hello {
            div[
                a = 1,
                b = "2",
                c? = true,
                d? = false,
                "e-f" = 3,
                {"g".to_string() + "-h"} = 4,
                i = None::<i32>,
                j = Some(5)
            ] {}
            "\n"
            br[k = 6];
            "\n"
            input[type = "text"];
        }
    }
}

mod e4 {
    markup::define! {
        Hello {
            .foo[a = 1] {
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
            {markup::raw("<span></span>")}
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
        Hello<'a, T: std::fmt::Debug, U>(
            arg: T,
            arg2: U,
            str: &'a str,
        ) where U: std::fmt::Display {
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
        Main {
            @const ONE: i32 = 1;
            @#[derive(Debug)] struct Int(i32);
            @let Int(two) = Int(2);
            @ONE @two "\n"
            @for x in &[1, 2, 3] {
                @let (_double, triple) = (x * 2, x * 3);
                {x} " * 3 = " {triple} "\n"
            }
        }
    }
}

mod e10 {
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

mod e11 {
    markup::define! {
        Classify(value: Option<i32>) {
            @if let Some(0) = *value {
                "Some(ZERO)"
            } else if let Some(value) = *(value) {
                "Some(" {value} ")"
            } else {
                "None"
            }
            "\n"
        }
        Main {
            {Classify { value: None }}
            {Classify { value: Some(0) }}
            {Classify { value: Some(1) }}
        }
    }
}

mod e12 {
    markup::define! {
        Classify(value: Option<i32>) {
            @match *value {
              Some(1) | Some(2) => {
                "1"
                " or 2"
              }
              Some(n) if n == 3 => {
                {n} {n}
              }
              Some(_) => {
                "Other"
              }
              None => {
                "None"
              }
            }
            "\n"
        }
        Main {
            {Classify { value: None }}
            {Classify { value: Some(0) }}
            {Classify { value: Some(1) }}
            {Classify { value: Some(2) }}
            {Classify { value: Some(3) }}
        }
    }
}

mod e13 {
    markup::define! {
        Main {
            @for i in 1..5 {
                {i} " * 2 = " {i * 2} ";\n"
            }
        }
    }
}

mod e14 {
    markup::define! {
        Main {
            {let x = 1;}
            {
                mod math {
                    pub fn add(x: i32, y: i32) -> i32 {
                        x + y
                    }
                }
            }
            {math::add(x, x)}
            @math::add(x, x)
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
    println!("{}\n", e11::Main {});
    println!("{}\n", e12::Main {});
    println!("{}\n", e13::Main {});
    println!("{}\n", e14::Main {});
}
