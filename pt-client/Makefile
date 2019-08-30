.PHONY : all
./dist/palit: ./src
	cargo build
	mv target/debug/DAW ./dist/palit
	test

.PHONY : test
test: 
	./scripts/test

.PHONY : clean
clean:
	rm -rf dist/*
	rm -rf target/*
