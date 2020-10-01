fn dynamic(data: &[usize]) -> impl std::fmt::Display + '_ {
    markup::dynamic! {
        foo {
            @for d in data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

fn to_string(data: &[usize]) -> String {
    markup::to_string! {
        foo {
            @for d in data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

fn to_writer(data: &[usize], writer: &mut impl std::fmt::Write) -> std::fmt::Result {
    markup::to_writer! {
        writer =>
        foo {
            @for d in data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

#[test]
fn t() {
    let data = (1..1000).into_iter().collect::<Vec<_>>();

    assert_eq!(dynamic(&data).to_string().len(), 23879);

    assert_eq!(to_string(&data).len(), 23879);

    let mut string = String::new();
    to_writer(&data, &mut string).unwrap();
    assert_eq!(string.len(), 23879);
}
