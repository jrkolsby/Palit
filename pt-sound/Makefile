.PHONY : prod
prod: linux
	./target/x86_64-unknown-linux-gnu/release/pt-sound NVidia 48000 1024

.PHONY : dev
dev: darwin-optimal
	./target/x86_64-apple-darwin/release/pt-sound

.PHONY : debug
debug: darwin
	./target/x86_64-apple-darwin/debug/pt-sound

darwin-optimal:
	cargo build --release --target x86_64-apple-darwin 

darwin:
	cargo build --target x86_64-apple-darwin 

linux:
	cargo build --release --target x86_64-unknown-linux-gnu

clean:
	rm -rf ./target/
