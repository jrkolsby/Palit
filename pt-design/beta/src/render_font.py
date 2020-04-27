from PIL import Image, ImageFont, ImageDraw
import cv2
import numpy as np
from shapely import geometry, ops

# FONT RENDERING

SIZE = 216
HOUGH_THRESH = 8
HOUGH_MIN_LEN = 10
HOUGH_MAX_GAP = 2
MAX_LINES = 70

font = ImageFont.truetype("fonts/OsakaMono.ttf", SIZE) # Load font
char_img = Image.new("L", (SIZE, SIZE), 0) #Initialize black bg
draw = ImageDraw.Draw(char_img)
# 人攴/~彡厂二工田冂冖丿艸匚爪巛州儿冫彳
draw.text((0, 0), ")", font=font, fill=(255)) # Draw white char

img = np.array(char_img) # Convert to numpy array
cv2.imwrite("bitmap.png", img)

# SKELETONIZATION and PATH DETECTION (perform hough transform)

kernel = np.ones((9, 9), np.uint8) 
# output = cv2.cvtColor(img, cv2.COLOR_GRAY2RGB)
lines = []
done = False

while not done:
    img = cv2.erode(img, kernel)  

    edges = cv2.Canny(img, 50, 150, apertureSize = 3)
    lines = cv2.HoughLinesP(edges, 1, np.pi/180, 
            HOUGH_THRESH,
            HOUGH_MIN_LEN,
            HOUGH_MAX_GAP)

    if len(lines) <= MAX_LINES:
        done = True

# SIMPLIFY LINE TOPOLOGY WITH MERGE

line_string = [] 
for line in lines:
    for x1, y1, x2, y2 in line:
        line_string.append(geometry.LineString([[x1,y1], [x2,y2]]))

multi_line = geometry.MultiLineString(line_string)

print(multi_line)

merged_line = ops.linemerge(multi_line)

print(merged_line)  # prints LINESTRING (0 0, 1 1, 2 2, 3 3)

# WRITE TO SVG (TODO: DATABASE)

f = open('path.svg', 'w+')
f.write('<svg width="{0}" height="{1}" xmlns="http://www.w3.org/2000/svg">'.format(SIZE, SIZE))


for line in merged_line:
    f.write('<path stroke="red" stroke-width="1" d="M')
    print('\n')
    for x, y in line.coords:
        print(x, y)
        f.write('{0} {1} '.format(int(x), int(y)))
        # cv2.line(output,(int(x1),int(y1)),(int(x2),int(y2)),(0,255,0),1)
    f.write('"/>')


f.write('"</svg>')
f.close()

# cv2.imwrite('output.png', output)
