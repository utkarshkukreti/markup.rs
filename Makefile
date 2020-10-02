default:
	erb README.md.erb > README.md
	cargo build
	cargo test

publish:
	cd markup-proc-macro && cargo publish
	sleep 5
	cargo search markup-proc-macro
	cd markup && cargo publish
