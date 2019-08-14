.PHONY : clean
clean: 
	docker image prune -f
	docker container prune -f
	docker-compose -f compose.yml down --remove-orphans --rmi 'all'

.PHONY : dev
dev:
	docker-compose -f compose.yml up -d --build
