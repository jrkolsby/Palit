.PHONY : dev
dev:
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && sudo make dev" && tmux split-window -v "cd pt-input && cargo run" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug	

.PHONY : demo
demo:
	tmux split-window -p 30 -v "tail -f /tmp/pt-debug" && tmux split-window -p 50 -v "cd pt-input && cargo run" && tmux split-window -p 1 -v "cd pt-sound && make dev" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug	

.PHONY : debug
debug:
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

.PHONY : modTest
modTest:
	gcc -c -fpic _plugin.c;
	gcc -shared -o testPlugin.so _plugin.o;

.PHONY : plugin
plugin: 
	faust -lang c -cn mydsp ./storage/modules/$(name).dsp > ./_plugin_part.c;
	cat ./pt-common/src/faust.h ./_plugin_part.c > _plugin.c;
	gcc -c -fpic _plugin.c;
	gcc -shared -o $(name).so _plugin.o;
	rm ./_plugin_part.c;
	rm ./_plugin.c;
	rm ./_plugin.o;
