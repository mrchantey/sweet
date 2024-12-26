set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments

default:
	just --list --unsorted

book:
	mdbook serve

expand *args:
	just watch 'cargo expand --test test_macro'
# just watch 'cargo expand --example scratch {{args}}'

install *args:
	cargo install --path ./cli {{args}}

test *args:
	just watch 'cargo test --lib -- {{args}}'


publish-all:
	just publish sweet_macros			| true
	just publish sweet 						| true
	just publish sweet-cli				| true

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

test-all-wasm *args:
	just test-wasm sweet --cargo=--features=bevy {{args}}

test-wasm crate *args:
	cargo run -p sweet-cli -- -p {{crate}} --example test_{{crate}}_wasm {{args}}


watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-i '**/*_g.rs' \
	-- {{command}}