pub fn escape(str: &str, w: &mut impl std::fmt::Write) -> std::fmt::Result {
    let mut last = 0;
    for (index, byte) in str.bytes().enumerate() {
        match byte {
            b'&' | b'<' | b'>' | b'"' => {
                w.write_str(&str[last..index])?;
                w.write_str(match byte {
                    b'&' => "&amp;",
                    b'<' => "&lt;",
                    b'>' => "&gt;",
                    _ => "&quot;",
                })?;
                last = index + 1;
            }
            _ => {}
        }
    }
    w.write_str(&str[last..])
}

#[test]
fn test() {
    let mut string = String::new();
    escape(
        r#"foo &<>" bar&bar<bar>bar"bar baz&&<<baz>>""baz"#,
        &mut string,
    )
    .unwrap();
    assert_eq!(
        string,
        r#"foo &amp;&lt;&gt;&quot; bar&amp;bar&lt;bar&gt;bar&quot;bar baz&amp;&amp;&lt;&lt;baz&gt;&gt;&quot;&quot;baz"#
    )
}
