// https://github.com/djc/template-benchmarks-rs/

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_teams, bench_big_table);
criterion_main!(benches);

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

fn bench_teams(c: &mut Criterion) {
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

    let mut group = c.benchmark_group("teams");
    let len = teams.to_string().len();
    assert_eq!(len, 239);
    group.throughput(Throughput::Bytes(len as u64));
    group.bench_function("teams", |b| b.iter(|| teams.to_string()));
    group.finish();
}

markup::define! {
    BigTable(table: Vec<Vec<usize>>) {
        table {
            @for r1 in table {
                tr {
                    @for r2 in r1 {
                        td { {r2} }
                    }
                }
            }
        }
    }
}

fn bench_big_table(c: &mut Criterion) {
    let size = 100;
    let table = (0..size).map(|_| (0..size).collect()).collect();
    let big_table = BigTable { table };

    let mut group = c.benchmark_group("big_table");
    let len = big_table.to_string().len();
    assert_eq!(len, 109915);
    group.throughput(Throughput::Bytes(len as u64));
    group.bench_function("big_table", |b| b.iter(|| big_table.to_string()));
    group.finish();
}
