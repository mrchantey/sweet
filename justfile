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

expand-rsx:
	just watch cargo expand -p sweet_rsx --example rsx_macro

hello-world *args:
	forky serve target/hello_world & \
	just watch 'just _hello-world {{args}}'

_hello-world *args:
	mkdir -p target/hello_world
	cp crates/sweet_rsx/examples/hello_world.html target/hello_world/hello_world.html
	cargo build -p sweet_rsx --example hello_world --target wasm32-unknown-unknown {{args}}
	wasm-bindgen \
	--out-dir ./target/hello_world \
	--out-name bindgen \
	--target web \
	~/.cargo_target/wasm32-unknown-unknown/debug/examples/hello_world.wasm