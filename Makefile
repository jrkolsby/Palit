homedir = /usr/local/palit

.PHONY : dev
dev: ipc
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && make dev" && tmux split-window -v "cd pt-input && make dev" && cd pt-client/ && sudo cargo run --release 2> /tmp/pt-debug

.PHONY : demo
demo: ipc
	cd pt-input && make dev &> /tmp/pt-debug &
	cd pt-sound && make dev &> /tmp/pt-debug &
	cd pt-client && cargo run --release 2> /tmp/pt-debut

.PHONY : debug
debug: ipc
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && sudo RUST_BACKTRACE=1 make debug" && tmux split-window -v "cd pt-input && RUST_BACKTRACE=1 cargo run" && cd pt-client/ && sudo RUST_BACKTRACE=1 cargo run 2> /tmp/pt-debug

.PHONY : prod
prod: 
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && make prod" && tmux split-window -v "cd pt-input && cargo build && sudo ./target/debug/pt-input" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug

.PHONY : tick
tick:
	watch -n .1 'printf %s TICK > /tmp/pt-client'

.PHONY : sound
sound:
	cd pt-sound && cargo run --release NVidia 48000 128

.PHONY : plugin
plugin: 
	faust -lang c -cn mydsp $(homedir)/modules/$(name).dsp > $(homedir)/modules/_plugin_part.c;
	cat $(homedir)/modules/faust.h $(homedir)/modules/_plugin_part.c > $(homedir)/modules/_plugin.c;
	gcc -c -fpic $(homedir)/modules/_plugin.c -o $(homedir)/modules/_plugin.o;
	gcc -shared -o $(homedir)/modules/$(name).so $(homedir)/modules/_plugin.o;
	rm $(homedir)/modules/_plugin_part.c;
	rm $(homedir)/modules/_plugin.c;
	rm $(homedir)/modules/_plugin.o;

.PHONY : ipc
ipc : 
	rm -f /tmp/pt-client
	rm -f /tmp/pt-sound
	rm -f /tmp/pt-debug
	mkfifo /tmp/pt-client
	mkfifo /tmp/pt-sound
	mkfifo /tmp/pt-debug

