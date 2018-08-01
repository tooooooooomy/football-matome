sakura: set-sakura-env

vagrant: set-vagrant-env

travis: set-travis-env

set-sakura-env:
	ln -snf ./submodules/local-git/.env.sakura ./.env

set-vagrant-env:
	ln -snf ./.env.vagrant ./.env

set-travis-env:
	ln -snf ./.env.travis ./.env

test:
	cargo test

test-debug:
	cargo test -- --nocapture

migrate:
	diesel migration run

rust-update:
	curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

lint:
	cargo build --features="clippy"
