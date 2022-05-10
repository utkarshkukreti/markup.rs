pub fn escape(str: &str, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
    let mut last = 0;
    for (index, byte) in str.bytes().enumerate() {
        macro_rules! go {
            ($expr:expr) => {{
                writer.write_str(&str[last..index])?;
                writer.write_str($expr)?;
                last = index + 1;
            }};
        }

        match byte {
            b'&' => go!("&amp;"),
            b'<' => go!("&lt;"),
            b'>' => go!("&gt;"),
            b'"' => go!("&quot;"),
            _ => {}
        }
    }
    writer.write_str(&str[last..])
}

pub struct Escape<'a, W>(pub &'a mut W);

impl<W: std::fmt::Write> std::fmt::Write for Escape<'_, W> {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        escape(s, &mut self.0)
    }
}

#[test]
fn test() {
    t("", "");
    t("<", "&lt;");
    t("a<", "a&lt;");
    t("<b", "&lt;b");
    t("a<b", "a&lt;b");
    t("a<>b", "a&lt;&gt;b");
    t("<>", "&lt;&gt;");
    t("≤", "≤");
    t("a≤", "a≤");
    t("≤b", "≤b");
    t("a≤b", "a≤b");
    t("a≤≥b", "a≤≥b");
    t("≤≥", "≤≥");
    t(
        r#"foo &<>" bar&bar<bar>bar"bar baz&&<<baz>>""baz"#,
        r#"foo &amp;&lt;&gt;&quot; bar&amp;bar&lt;bar&gt;bar&quot;bar baz&amp;&amp;&lt;&lt;baz&gt;&gt;&quot;&quot;baz"#,
    );

    fn t(input: &str, output: &str) {
        let mut string = String::new();
        escape(input, &mut string).unwrap();
        assert_eq!(string, output);
    }
}

#[test]
fn test_arguments() {
    use std::fmt::Write;

    t("", "&quot;&quot;");
    t("<", "&quot;&lt;&quot;");
    t("a<", "&quot;a&lt;&quot;");
    t("<b", "&quot;&lt;b&quot;");
    t("a<b", "&quot;a&lt;b&quot;");
    t("a<>b", "&quot;a&lt;&gt;b&quot;");
    t("<>", "&quot;&lt;&gt;&quot;");
    t("≤", "&quot;≤&quot;");
    t("a≤", "&quot;a≤&quot;");
    t("≤b", "&quot;≤b&quot;");
    t("a≤b", "&quot;a≤b&quot;");
    t("a≤≥b", "&quot;a≤≥b&quot;");
    t("≤≥", "&quot;≤≥&quot;");
    t(
        r#"foo &<>" bar&bar<bar>bar"bar baz&&<<baz>>""baz"#,
        r#"&quot;foo &amp;&lt;&gt;\&quot; bar&amp;bar&lt;bar&gt;bar\&quot;bar baz&amp;&amp;&lt;&lt;baz&gt;&gt;\&quot;\&quot;baz&quot;"#,
    );
    t('<', "'&lt;'");

    fn t(input: impl std::fmt::Debug, output: &str) {
        let mut string = String::new();
        write!(Escape(&mut string), "{}", format_args!("{:?}", input)).unwrap();
        assert_eq!(string, output);
    }
}
