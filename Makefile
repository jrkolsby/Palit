.PHONY : dev
dev:
	tmux split-window -v "cd pt-sound && make dev" && tmux split-window -v "tail -f /tmp/pt-debug" && tmux split-window -v "cd pt-input && cargo run" && cd pt-client/ && cargo run 2> /tmp/pt-debug	

.PHONY : tick
tick:
	watch -n .1 'printf %s TICK > /tmp/pt-client'

.PHONY : sound
sound:
	cd pt-sound && cargo run --release NVidia 48000 128
