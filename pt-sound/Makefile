.PHONY : prod
prod: release
	./target/release/pt-sound NVidia 48000 1024

.PHONY : dev
dev: debug
	./target/debug/pt-sound

debug:
	cargo build --target x86_64-apple-darwin 

release:
	cargo build --release --target x86_64-unknown-linux-gnu


