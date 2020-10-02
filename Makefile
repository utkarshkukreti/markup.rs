default:
	erb README.md.erb > README.md
	erb markup/tests/macros.rs.erb > markup/tests/macros.rs
	rustfmt markup/tests/macros.rs
	cargo fmt -- --check
	cargo build
	cargo test

publish:
	cd markup-proc-macro && cargo publish
	sleep 5
	cargo search markup-proc-macro
	cd markup && cargo publish
