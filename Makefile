.PHONY : dev
dev:
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && make dev" && tmux split-window -v "cd pt-input && cargo run" && cd pt-client/ && cargo run 2> /tmp/pt-debug	

.PHONY : debug
debug:
	tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-sound && RUST_BACKTRACE=1 make dev" && tmux split-window -v "cd pt-input && RUST_BACKTRACE=1 cargo run" && cd pt-client/ && RUST_BACKTRACE=1 cargo run 2> /tmp/pt-debug	

.PHONY : tick
tick:
	watch -n .1 'printf %s TICK > /tmp/pt-client'

.PHONY : sound
sound:
	cd pt-sound && cargo run --release NVidia 48000 128
