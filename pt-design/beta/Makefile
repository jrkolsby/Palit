.PHONY : all
all:
	python3 src/render_font.py

.PHONY : svg
svg:
	fontforge -c 'import fontforge;fontforge.open("./fonts/OsakaMono.ttf").generate("OsakaMono.svg")'
