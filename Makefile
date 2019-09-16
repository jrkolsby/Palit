.PHONY : clean
clean: 
	docker image prune -f
	docker container prune -f
	docker-compose -f compose.yml down --remove-orphans --rmi 'all'

.PHONY : dev
dev:
	docker-compose -f compose.yml up -d -V --build
	docker exec -it palit_pt-sound-dev_1 /bin/bash

.PHONY : run
run:
	cd ./pt-client && ./scripts/run

.PHONY : tick
tick:
	watch -n .1 "echo TICK > /tmp/pt-client"
