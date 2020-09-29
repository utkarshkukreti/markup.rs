use insta::assert_display_snapshot;

#[test]
fn t() {
    markup::define! {
      T1 { "Hello" }
      T2 { "Hello" ", World!" }
    }
    assert_display_snapshot!(T1 {});
    assert_display_snapshot!(T2 {});
}
