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
        }
    },
    A {} => "3π36&lt;&gt;",
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
    },
    A {} => r#"<foo id="bar"><baz id="foo" class="quux 1 5"></baz><bar id="4" class="-1">7</bar></foo>"#,
}

t! {
    t6,
    {
        A {
            div [
                a = 1,
                b = "2",
                c? = true,
                d? = false,
                "e-f" = 3,
                {"g".to_string() + "-h"} = 4,
                i = None::<i32>,
                j = Some(5)
            ] {}
            br[k = 6];
        }
    },
    A {} => r#"<div a="1" b="2" c e-f="3" g-h="4" j="5"></div><br k="6">"#,
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
            v: V,
        ) where U: std::fmt::Display {
            div {
                {format!("{:?}", arg)}
                {format!("{}", arg2)}
                {str}
                {v}
            }
        }
    },
    A {
        arg: (1, 2),
        arg2: "arg2",
        str: "str",
        v: markup::dynamic!(foo {})
    } => "<div>(1, 2)arg2str<foo></foo></div>",
}
