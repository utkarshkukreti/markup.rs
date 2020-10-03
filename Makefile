default:
	erb README.md.erb > README.md
	cargo fmt -- --check
	cargo build
	cargo test
	cargo test --examples

publish:
	cd markup-proc-macro && cargo publish
	sleep 10
	cargo search markup-proc-macro
	cd markup && cargo publish
