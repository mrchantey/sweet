set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

default:
	just --list --unsorted

# just watch 'cargo expand --example scratch {{args}}'

#💡 CLI

cli *args:
	cargo run -p sweet-cli -- {{args}}

install-cli *args:
	cargo install --path crates/sweet-cli {{args}}

run crate example *args:
	just watch 'cargo run -p {{crate}} --example {{example}} {{args}}'
run-wasm crate example *args:
	just watch 'cargo run -p {{crate}} --example {{example}} --target wasm32-unknown-unknown {{args}}'



clean-analyzer:
	rm -rf $CARGO_TARGET_DIR/rust-analyzer

#💡 Test

test-cli *args:
	just watch 'cargo test -p sweet-cli --lib -- --watch {{args}}'


# could possibly be a cli command
test crate *args:
	just watch 'cargo test -p {{crate}} --lib -- --watch {{args}}'
test-e2e crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} -- --watch {{args}}'
test-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --all-features -- {{args}}'
test-wasm crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown -- --watch {{args}}'
test-wasm-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown --all-features -- {{args}}'
test-wasm-e2e crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} --target wasm32-unknown-unknown -- --watch {{args}}'

test-all *args:
	cargo test --workspace -- {{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p sweet_utils	-- {{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p sweet_test   -- {{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p sweet_bevy   -- {{args}}
	cargo test --target wasm32-unknown-unknown --all-features -p sweet_web   	-- {{args}}

# upstream from sweet_test
test-fs *args:
	just watch cargo test -p sweet_fs --lib {{args}}
# upstream from sweet_test
test-utils *args:
	just watch cargo test -p sweet_utils --lib {{args}}

expand-wasm test *args:
	just watch 'cargo expand --test {{test}} --target wasm32-unknown-unknown {{args}}'
expand test *args:
	just watch 'cargo expand --test {{test}} {{args}}'

#💡 Publish

# order matters
publish-all *args:
	just publish sweet_utils				{{args}} | true
	just publish sweet_fs						{{args}} | true
	just publish sweet_test_macros	{{args}} | true
	just publish sweet_test					{{args}} | true
	just publish sweet_server				{{args}} | true
	just publish sweet_web					{{args}} | true
	just publish sweet_bevy					{{args}} | true
	just publish sweet 							{{args}} | true
	just publish sweet-cli					{{args}} | true

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 1

build-wasm crate example *args:
	cargo build -p {{crate}} --example {{example}} --target wasm32-unknown-unknown {{args}}
	wasm-bindgen \
	--out-dir ./target/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	~/.cargo_target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm

watch *command:
	forky watch --rusty	-- {{command}}
