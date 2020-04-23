from PIL import Image, ImageFont, ImageDraw

image = Image.new("L", (100, 100), 255)

draw = ImageDraw.Draw(image)

# use a truetype font

font = ImageFont.truetype("fonts/ArialUnicode.ttf", 15)

draw.text((10, 25), "艸", font=font)

image.save("bitmap.png")
