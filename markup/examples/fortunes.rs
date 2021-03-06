pub struct Fortune {
    pub id: i32,
    pub message: &'static str,
}

markup::define! {
    Define<'a>(fortunes: &'a [Fortune]) {
        @let fortunes = *fortunes;
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }
    }
}

pub fn new(fortunes: &[Fortune]) -> impl std::fmt::Display + '_ {
    markup::new! {
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }
    }
}

pub static FORTUNES: &[Fortune] = &[
    Fortune {
        id: 1,
        message: "fortune: No such file or directory",
    },
    Fortune {
        id: 2,
        message: "A computer scientist is someone who fixes things that aren\'t broken.",
    },
    Fortune {
        id: 3,
        message: "After enough decimal places, nobody gives a damn.",
    },
    Fortune {
        id: 4,
        message: "A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1",
    },
    Fortune {
        id: 5,
        message: "A computer program does what you tell it to do, not what you want it to do.",
    },
    Fortune {
        id: 6,
        message: "Emacs is a nice operating system, but I prefer UNIX. — Tom Christaensen",
    },
    Fortune {
        id: 7,
        message: "Any program that runs right is obsolete.",
    },
    Fortune {
        id: 8,
        message: "A list is only as strong as its weakest link. — Donald Knuth",
    },
    Fortune {
        id: 9,
        message: "Feature: A bug with seniority.",
    },
    Fortune {
        id: 10,
        message: "Computers make very fast, very accurate mistakes.",
    },
    Fortune {
        id: 11,
        message:
            "<script>alert(\"This should not be displayed in a browser alert box.\");</script>",
    },
    Fortune {
        id: 12,
        message: "フレームワークのベンチマーク",
    },
];

#[allow(dead_code)]
pub fn main() {
    println!("{}", Define { fortunes: FORTUNES });
}

#[test]
fn t() {
    let define = Define { fortunes: FORTUNES }.to_string();
    let new = new(FORTUNES).to_string();
    assert_eq!(define.len(), 1153);
    assert_eq!(define, new);
}
