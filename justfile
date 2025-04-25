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
test-e2e crate *args:
	just watch 'cargo test -p {{crate}} --lib --features=e2e -- --e2e --watch {{args}}'
test-integration crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} -- --watch {{args}}'
test-doc crate *args:
	just watch 'cargo test -p {{crate}} --doc -- {{args}}'
test-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --all-features -- {{args}}'
test-wasm crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown -- --watch {{args}}'
test-wasm-feat crate *args:
	just watch 'cargo test -p {{crate}} --lib --target wasm32-unknown-unknown --all-features -- {{args}}'
test-wasm-e2e crate test_name *args:
	just watch 'cargo test -p {{crate}} --test {{test_name}} --target wasm32-unknown-unknown -- --watch {{args}}'

# really hard to test sweet_utils wasm as is upstream of sweet_test
# good news is at least compilation is tested because sweet_test depends on it
# cargo test --lib --target wasm32-unknown-unknown --all-features -p sweet_utils	-- {{args}}
test-all *args:
	cargo test --workspace {{args}}
	cargo test -p sweet_bevy 				--all-features -- {{args}}
	cargo test -p sweet_fs 					--all-features -- {{args}}
	cargo test -p sweet_net 				--all-features -- {{args}}
	cargo test -p sweet_server 			--all-features -- {{args}}
	cargo test -p sweet_test --lib 	--all-features -- --e2e {{args}}
	cargo test -p sweet_utils 			--all-features -- {{args}}
	cargo test -p sweet-cli 				--all-features -- {{args}}
	cargo test --lib --target wasm32-unknown-unknown --all-features -p sweet_bevy   -- {{args}}
	cargo test --lib --target wasm32-unknown-unknown --all-features -p sweet_test   -- {{args}}
	cargo test --lib --target wasm32-unknown-unknown --all-features -p sweet_web   	-- {{args}}

# upstream from sweet_test
test-fs *args:
	just watch 'cargo test -p sweet_fs --lib {{args}}'
# upstream from sweet_test
test-utils *args:
	just watch 'cargo test -p sweet_utils --lib {{args}}'

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
	just publish sweet_net					{{args}} | true
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
	sweet watch \
	--include "**/*.rs" \
	--exclude "{.git,target,html}/**" \
	--cmd "{{command}}"




# creates a directory ~/chrome-for-testing and installs chrome and chromedriver there.
# The latest version can be found at https://googlechromelabs.github.io/chrome-for-testing/
# Previous versions can be found at
install-chromedriver:
	wget https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.114/linux64/chrome-linux64.zip -P ~/chrome-for-testing
	wget https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.114/linux64/chromedriver-linux64.zip -P ~/chrome-for-testing
	mkdir -p ~/chrome-for-testing
	unzip ~/chrome-for-testing/chrome-linux64.zip -d ~/chrome-for-testing
	unzip ~/chrome-for-testing/chromedriver-linux64.zip -d ~/chrome-for-testing
	chmod +x ~/chrome-for-testing/chromedriver-linux64/chromedriver
	export PATH="$PATH:$HOME/chrome-for-testing/chromedriver-linux64"
	echo "Chrome installed at: $HOME/chrome-for-testing/chrome-linux64/chrome"
	echo "ChromeDriver installed at: $HOME/chrome-for-testing/chromedriver-linux64/chromedriver"
	echo "ChromeDriver version:"
	~/chrome-for-testing/chromedriver-linux64/chromedriver --version

search *args:
	cargo search {{args}} | head -n 1
