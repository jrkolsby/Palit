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
	cp -f ./storage/bin/* ./dist/bin/

.PHONY : dist-debug
dist-debug: clean
	cd ./pt-client/ && cargo build
	cd ./pt-sound/ && cargo build
	cd ./pt-input/ && make sniffMk;
	mv -f ./pt-client/target/debug/pt-client ./dist/bin/;
	mv -f ./pt-sound/target/debug/pt-sound ./dist/bin/;
	mv -f ./pt-input/bin/sniffMk ./dist/bin/pt-input;
	cp -f ./storage/bin/* ./dist/bin/

.PHONY : dev
dev: ipc dist
	tmux set remain-on-exit on && \
	tmux split-window -v "cat /tmp/pt-debug" && \
	tmux split-window -v "cd storage && ../dist/bin/pt-sound" && \
	tmux split-window -v "cd storage && sudo ../dist/bin/pt-input 1> /tmp/pt-client 2> /tmp/pt-sound" && \
	cd storage && ../dist/bin/pt-client 2> /tmp/pt-debug

.PHONY : debug
debug: ipc dist-debug
	tmux set remain-on-exit on && \
	tmux split-window -v "cat /tmp/pt-debug" && \
	tmux split-window -v "cd storage && RUST_BACKTRACE=1 ../dist/bin/pt-sound" && \
	tmux split-window -v "cd storage && sudo RUST_BACKTRACE=1 ../dist/bin/pt-input 1> /tmp/pt-client 2> /tmp/pt-sound" && \
	cd storage && RUST_BACKTRACE=1 ../dist/bin/pt-client 2> /tmp/pt-debug

.PHONY : ipc
ipc : 
	rm -f /tmp/pt-client
	rm -f /tmp/pt-sound
	rm -f /tmp/pt-debug
	mkfifo /tmp/pt-client
	mkfifo /tmp/pt-sound
	mkfifo /tmp/pt-debug

