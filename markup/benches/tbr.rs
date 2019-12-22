// https://github.com/djc/template-benchmarks-rs/
#![feature(test)]

extern crate test;

pub struct Team {
    name: String,
    score: u8,
}

markup::define! {
    Teams(year: u16, teams: Vec<Team>) {
        html {
            head {
                title { {year} }
            }
            body {
                h1 {
                    "CSL " {year}
                }
                ul {
                    @for (index, team) in teams.iter().enumerate() {
                        li.{if index == 0 { Some("champion") } else { None } } {
                            b { {team.name} } ": " {team.score}
                        }
                    }
                }
            }
        }
    }
}

#[bench]
fn bench_teams(b: &mut test::Bencher) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };
    b.iter(|| teams.to_string());
    b.bytes = teams.to_string().len() as u64;
}

markup::define! {
    BigTable(table: Vec<Vec<usize>>) {
        table {
            @for r1 in table.iter() {
                tr {
                    @for r2 in r1.iter() {
                        td { {r2} }
                    }
                }
            }
        }
    }
}

#[bench]
fn bench_big_table(b: &mut test::Bencher) {
    let size = 100;
    let table = (0..size).map(|_| (0..size).collect()).collect();
    let big_table = BigTable { table };
    b.iter(|| big_table.to_string());
    b.bytes = big_table.to_string().len() as u64;
}
