set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

default:
	just --list --unsorted

# just watch 'cargo expand --example scratch {{args}}'

#💡 CLI

cli *args:
	cargo run -p sweet-cli -- {{args}}

install-cli *args:
	cargo install --path ./cli {{args}}

run example *args:
	just watch 'cargo run --example {{example}} {{args}}'
run-wasm example *args:
	just watch 'cargo run --example {{example}} --target wasm32-unknown-unknown {{args}}'

clear-deno:
	rm -rf ./target/sweet/deno.ts

#💡 Test

test-cli *args:
	just watch 'cargo test -p sweet-cli --lib -- --watch {{args}}'

test crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} -- --watch {{args}}'
test-lib crate *args:
	just watch 'cargo test -p {{crate}} --lib -- --watch {{args}}'

test-wasm crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} --target wasm32-unknown-unknown -- --watch {{args}}'
test-wasm-lib crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown -- --watch {{args}}'

test-all *args:
	cargo test -p sweet_test --test hello_test -- {{args}}
	cargo test -p sweet_test --test hello_test --target wasm32-unknown-unknown -- {{args}}
	cargo test -p sweet_test --test hello_async -- {{args}}
	cargo test -p sweet_test --test hello_async --target wasm32-unknown-unknown -- {{args}}
	cargo test --workspace -- {{args}}
	cargo test -p sweet_rsx --lib --target wasm32-unknown-unknown -- {{args}}
	cargo test -p sweet_test --lib --target wasm32-unknown-unknown -- {{args}}
	cargo test -p sweet_test --test macros --target wasm32-unknown-unknown -- {{args}}

expand-wasm test *args:
	just watch 'cargo expand --test {{test}} --target wasm32-unknown-unknown {{args}}'
expand test *args:
	just watch 'cargo expand --test {{test}} {{args}}'

#💡 Publish

publish-all:
	just publish sweet_rsx_macros		| true
	just publish sweet_rsx					| true
	just publish sweet_test_macros	| true
	just publish sweet_test					| true
	just publish sweet 							| true
	just publish sweet-cli					| true

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 1

build-wasm example *args:
	cargo build --example {{example}} --target wasm32-unknown-unknown {{args}}
	wasm-bindgen \
	--out-dir ./target/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	~/.cargo_target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm

test-runner test-binary *args:
	wasm-bindgen \
	--out-dir ./target/sweet \
	--out-name bindgen \
	--target web \
	--no-typescript \
	{{test-binary}}
	deno --allow-read run.ts {{args}}

watch *command:
	forky watch --rusty	-- {{command}}

expand-rsx:
	just watch cargo expand -p sweet_rsx --example rsx_macro