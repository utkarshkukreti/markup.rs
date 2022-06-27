macro_rules! t {
    ($name:ident, {$($define:tt)+}, $($eval:expr => $expect:expr,)+) => {
        #[test]
        fn $name() {
            markup::define! {
                $($define)+
            }
            $(
                assert_eq!($eval.to_string(), $expect);
            )+
        }
    };
}

t! {
    t1,
    {
        A { }
        B { 1 }
        C { 2 3 }
        D { 4 "5" 6 }
        E { 7 " < " 8 }
        F { 9 " ≤ " 10 }
    },
    A {} => "",
    B {} => "1",
    C {} => "23",
    D {} => "456",
    E {} => "7 &lt; 8",
    F {} => "9 ≤ 10",
}

t! {
    t2,
    {
        A {
            {1 + 2}
            {'π'}
            {if true { Some(3) } else { Some(4) }}
            {if false { Some(5) } else { Some(6) }}
            {"<>"}
            {true}
            {false}
        }
    },
    A {} => "3π36&lt;&gt;truefalse",
}

t! {
    t3,
    {
        A {
            div {}
            br;
        }
    },
    A {} => "<div></div><br>",
}

t! {
    t4,
    {
        A {
            1
            .foo {
                2
                .bar.baz {
                    3
                    #quux {
                        4
                    }
                    5
                }
                6
            }
            7
        }
    },
    A {} => r#"1<div class="foo">2<div class="bar baz">3<div id="quux">4</div>5</div>6</div>7"#,
}

t! {
    t5,
    {
        A {
            foo#bar {
                baz.quux#"foo".{1}.{2 + 3} {}
                bar#{4}.{5 - 6} { 7 }
            }
        }
        B {
            foo "bar"
            foo#bar "baz"
            foo.bar[baz = true] "quux"
            foo.bar[baz = true]; "quux"
        }
        C {
            foo[fn = true, async = false, mod = true, r#move = true] {}
        }
    },
    A {} => r#"<foo id="bar"><baz id="foo" class="quux 1 5"></baz><bar id="4" class="-1">7</bar></foo>"#,
    B {} => r#"<foo>bar</foo><foo id="bar">baz</foo><foo class="bar" baz>quux</foo><foo class="bar" baz>quux"#,
    C {} => r#"<foo fn mod move></foo>"#,
}

t! {
    t6,
    {
        A {
            div [
                a = 1,
                b = "2",
                c = true,
                c2 = &true,
                c3 = &&true,
                c4,
                d = false,
                d2 = &false,
                d3 = &&false,
                "e-f" = 3,
                {"g".to_string() + "-h"} = 4,
                i = None::<i32>,
                i2 = &None::<i32>,
                i3 = &&None::<i32>,
                j = Some(5),
                j2 = &Some(5),
                j3 = &&Some(5),
                h = (6, ("7", String::from("8"), true, Some(false), None::<bool>)),
            ] {}
            br[k = 6];
        }
    },
    A {} => r#"<div a="1" b="2" c c2 c3 c4 e-f="3" g-h="4" j="5" j2="5" j3="5" h="678truefalse"></div><br k="6">"#,
}

t! {
    t7,
    {
        A(foo: u32, bar: i32, baz: String) {
            {foo} {bar} {*foo as i32 + bar} {baz}
        }
    },
    A { foo: 1, bar: -2, baz: "3".into() } => "1-2-13",
}

t! {
    t8,
    {
        A<'a, T: std::fmt::Debug, U, V: markup::Render>(
            arg: T,
            arg2: U,
            str: &'a str,
            box_str: Box<str>,
            v: V,
        ) where U: std::fmt::Display {
            div {
                {format!("{:?}", arg)}
                {format!("{}", arg2)}
                {str}
                {box_str}
                {v}
            }
        }
    },
    A {
        arg: (1, 2),
        arg2: "arg2",
        str: "str",
        box_str: "box_str".into(),
        v: markup::new!(foo {})
    } => "<div>(1, 2)arg2strbox_str<foo></foo></div>",
}

t! {
    t9,
    {
        A<Bar: Fn(i32) -> String>(foo: i32, bar: Bar, baz: B) {
            @foo
            @bar(*foo + 1)
            @baz.foo
            @format!("{} {} {}", foo, bar(*foo + 2), baz.foo + 3)
            @B { foo: foo + 4 }
        }
        B(foo: i32) {
            @foo @foo @foo
        }
    },
    A {
        foo: 1,
        bar: |x| (x + 2).to_string(),
        baz: B { foo: 3 },
    } => "1431 5 6555",
}

t! {
    t10,
    {
        A {
            @fn foo() -> i32 { 1 }
            @mod bar {
                pub fn baz() -> i32 { 2 }
            }
            @const QUUX: i32 = 3;
            @static FOUR: i32 = 4;

            @foo()
            @bar::baz()
            @QUUX
            @FOUR

            @#[derive(Debug)] struct Int(i32);
            @let Int(five) = Int(5);
            @five
            @{6}
            @{6 + 1}
            @{let eight = Int(8); eight.0}

            @crate::add(10, -1)

            @self::add(10, -2)
        }
    },
    A {} => "1234567898",
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

t! {
    t11,
    {
        A {
            @let foos = [None, Some(1), Some(2), Some(3)];
            @for (index, foo) in foos.iter().enumerate() {
                "index=" @index " :: "
                @B { foo: *foo } ";"
                @C { foo: *foo } ";"
                @D { foo: *foo } ";"
                @E { foo: *foo } "\n"
            }
        }
        B(foo: Option<i32>) {
            @if foo.is_some() {
                @let foo = foo.unwrap();
                @if foo == 1 {
                    "ONE"
                } else if foo == 2 {
                    "TWO"
                } else {
                    "OTHER"
                }
            } else {
                "NONE"
            }
        }
        C(foo: Option<i32>) {
            @if *foo == Some(1) {
                "ONE"
            } else if *foo == Some(2) {
                "TWO"
            } else if foo.is_some() {
                "OTHER"
            } else {
                "NONE"
            }
        }
        D(foo: Option<i32>) {
            @if let Some(1) = foo {
                "ONE"
            } else if let Some(2) = foo {
                "TWO"
            } else if let Some(_) = foo {
                "OTHER"
            } else {
                "NONE"
            }
        }
        E(foo: Option<i32>) {
            @match foo {
                Some(1) => { "ONE" }
                Some(n) if *n == 2 => { "TWO" }
                Some(_) => { "OTHER" }
                None => { "NONE" }
            }
        }
    },
    A {} => "\
index=0 :: NONE;NONE;NONE;NONE
index=1 :: ONE;ONE;ONE;ONE
index=2 :: TWO;TWO;TWO;TWO
index=3 :: OTHER;OTHER;OTHER;OTHER
",
}

t! {
    t12,
    {
        A(b: B) {
            @b
            @format!("{:?}", b)
        }
        #[derive(Clone, Debug)]
        B(foo: i32, bar: char) {
            @foo @bar
        }
    },
    A { b: B { foo: 1, bar: '?' }.clone() } => "1?B { foo: 1, bar: '?' }",
}

t! {
    t13,
    {
        A() {
            $"foo-bar";
            $"foo-bar" {}
            $"foo-bar"[baz = "quux"] {}
        }
        B(name: &'static str) {
            ${name};
            ${name} {}
        }
    },
    A {} => r#"<foo-bar><foo-bar></foo-bar><foo-bar baz="quux"></foo-bar>"#,
    B { name: "foo-bar" } => r#"<foo-bar><foo-bar></foo-bar>"#,
}
