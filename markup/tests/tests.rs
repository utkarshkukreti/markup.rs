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
