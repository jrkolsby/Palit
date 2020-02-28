.PHONY : clean
clean:
	rm -f ./dist/bin/pt-client;
	rm -f ./dist/bin/pt-sound;
	rm -f ./dist/bin/pt-input;

.PHONY : dist
dist: clean
	cd ./pt-client/ && cargo build --release;
	cd ./pt-sound/ && cargo build --release
	cd ./pt-input/ && make sniffMk;
	mv -f ./pt-client/target/release/pt-client ./dist/bin/;
	mv -f ./pt-sound/target/release/pt-sound ./dist/bin/;
	mv -f ./pt-input/bin/sniffMk ./dist/bin/pt-input;

.PHONY : dev
dev: ipc dist
	tmux split-window -v "cat /tmp/pt-debug" && \
	tmux split-window -v "cd storage && ../dist/bin/pt-sound" && \
	tmux split-window -v "cd storage && sudo ../dist/bin/pt-input 1> /tmp/pt-client 2> /tmp/pt-sound" && \
	cd storage && ../dist/bin/pt-client 2> /tmp/pt-debug

.PHONY : debug
debug: ipc
	tmux split-window -v "cat /tmp/pt-debug" && \
	tmux split-window -v "cd pt-sound && sudo RUST_BACKTRACE=1 make debug" && \
	tmux split-window -v "cd pt-input && RUST_BACKTRACE=1 cargo run" && \
	cd pt-client/ && sudo RUST_BACKTRACE=1 cargo run 2> /tmp/pt-debug

.PHONY : prod
prod: 
	tmux split-window -v "cat /tmp/pt-debug" && \
	tmux split-window -v "cd pt-sound && make prod" && \
	tmux split-window -v "cd pt-input && cargo build && sudo ./target/debug/pt-input" && \
	cd pt-client/ && cargo run --release 2> /tmp/pt-debug

.PHONY : sound
sound:
	cd pt-sound && cargo run --release NVidia 48000 128

.PHONY : ipc
ipc : 
	rm -f /tmp/pt-client
	rm -f /tmp/pt-sound
	rm -f /tmp/pt-debug
	mkfifo /tmp/pt-client
	mkfifo /tmp/pt-sound
	mkfifo /tmp/pt-debug

