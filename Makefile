.PHONY : dev
dev:
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && sudo make dev" && tmux split-window -v "cd pt-input && cargo run" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug	

.PHONY : demo
demo:
	tmux split-window -p 30 -v "tail -f /tmp/pt-debug" && tmux split-window -p 50 -v "cd pt-input && cargo run" && tmux split-window -p 1 -v "cd pt-sound && make dev" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug	

.PHONY : debug
debug:
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && RUST_BACKTRACE=1 sudo make debug" && tmux split-window -v "cd pt-input && RUST_BACKTRACE=1 cargo run" && cd pt-client/ && RUST_BACKTRACE=1 sudo cargo run 2> /tmp/pt-debug	

.PHONY : prod
prod: 
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && make prod" && tmux split-window -v "cd pt-input && cargo build && sudo ./target/debug/pt-input" && cd pt-client/ && cargo run --release 2> /tmp/pt-debug	

.PHONY : tick
tick:
	watch -n .1 'printf %s TICK > /tmp/pt-client'

.PHONY : sound
sound:
	cd pt-sound && cargo run --release NVidia 48000 128
