.PHONY : prod
prod: release
	./target/x86_64-unknown-linux-gnu/release/pt-sound NVidia 48000 1024

.PHONY : dev
dev: clean debug
	./target/x86_64-apple-darwin/debug/pt-sound

debug:
	cargo build --target x86_64-apple-darwin 

release:
	cargo build --release --target x86_64-unknown-linux-gnu

clean:
	rm -rf ./target/
