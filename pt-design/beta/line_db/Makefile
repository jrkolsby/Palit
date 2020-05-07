.PHONY : all
all: clean
	python3 src/render_font.py

.PHONY : svg
svg:
	fontforge -c 'import fontforge;fontforge.open("./fonts/OsakaMono.ttf").generate("OsakaMono.svg")'

.PHONY : clean
clean: 
	rm -f ./*.png
	rm -f ./*.xml
	rm -f ./*.svg
