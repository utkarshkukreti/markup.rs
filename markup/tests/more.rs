markup::define! {
    Define<'a>(data: &'a [usize]) {
        foo {
            @for d in *data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

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

    let define = Define { data: &data }.to_string();
    let dynamic = dynamic(&data).to_string();
    let to_string = to_string(&data);
    let to_writer = {
        let mut string = String::new();
        to_writer(&data, &mut string).unwrap();
        string
    };

    assert_eq!(define.len(), 23879);
    assert_eq!(define, dynamic);
    assert_eq!(define, to_string);
    assert_eq!(define, to_writer);
}
