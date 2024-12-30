set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

default:
	just --list --unsorted

book:
	mdbook serve

expand-wasm test *args:
	just watch 'cargo expand --test {{test}} --target wasm32-unknown-unknown {{args}}'
expand test *args:
	just watch 'cargo expand --test {{test}} {{args}}'
# just watch 'cargo expand --example scratch {{args}}'

install *args:
	cargo install --path ./cli {{args}}

test test_name *args:
	just watch 'cargo test --test {{test_name}} -- --watch {{args}}'
test-wasm test_name *args:
	just watch 'cargo test --test {{test_name}} --target wasm32-unknown-unknown -- --watch {{args}}'

test-all *args:
	cargo test --test hello_test -- {{args}}
	cargo test --test hello_test --target wasm32-unknown-unknown -- {{args}}
	cargo test --test hello_async -- {{args}}
	cargo test --test hello_async --target wasm32-unknown-unknown -- {{args}}


publish-all:
	just publish sweet_macros			| true
	just publish sweet 						| true
	just publish sweet-cli				| true

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

# test-all-wasm *args:
# 	just test-wasm sweet --cargo=--features=bevy {{args}}

# test-wasm crate *args:
# 	cargo run -p sweet-cli -- -p {{crate}} --example test_{{crate}}_wasm {{args}}

build-wasm example *args:
	cargo build --example {{example}} --target wasm32-unknown-unknown {{args}}
	wasm-bindgen \
	--out-dir ./target/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	~/.cargo_target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm

run-wasm example *args:
	just build-wasm {{example}} {{args}}
	deno --allow-read run.ts
# wasmtime ./target/wasm/bindgen_bg.wasm

test-runner test-binary *args:
	wasm-bindgen \
	--out-dir ./target/wasm \
	--out-name bindgen \
	--target web \
	--no-typescript \
	{{test-binary}}
	deno --allow-read run.ts {{args}}

watch *command:
	forky watch \
	-w '**/*.rs' \
	-w '**/*.ts' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-i '**/*_g.rs' \
	-- {{command}}