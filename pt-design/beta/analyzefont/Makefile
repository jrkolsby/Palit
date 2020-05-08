.PHONY : all
all: clean
	for font in ./fonts/*; do python3 src/analyze.py $$font; done

.PHONY : clean
clean: 
	rm -f ./*.png
	rm -f ./*.xml
	rm -f ./*.svg
