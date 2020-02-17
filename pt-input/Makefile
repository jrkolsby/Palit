.PHONY : dev
dev: sniffMk
	sudo ./bin/sniffMk 1> /tmp/pt-client 2> /tmp/pt-sound

.PHONY : sniffMk
sniffMk: 
	clang -o ./bin/sniffMK ./src/sniffMK.m -framework Cocoa -framework Carbon

.PHONY : prod
prod: 
	cargo build
	sudo ./target/debug/pt-input
