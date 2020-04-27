from PIL import Image, ImageFont, ImageDraw
import cv2
import numpy as np

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
draw.text((0, 0), "~", font=font, fill=(255)) # Draw white char

img = np.array(char_img) # Convert to numpy array
cv2.imwrite("bitmap.png", img)

'''
size = np.size(img)
skel = np.zeros(img.shape,np.uint8)

element = cv2.getStructuringElement(cv2.MORPH_CROSS,(3,3))
done = False

while(not done):
    eroded = cv2.erode(img,element)
    temp = cv2.dilate(eroded,element)
    temp = cv2.subtract(img,temp)
    skel = cv2.bitwise_or(skel,temp)
    img = eroded.copy()

    zeros = size - cv2.countNonZero(img)
    if zeros == size:
        done = True

cv2.imwrite("skeleton.png", skel)
'''

# SKELETONIZATION and PATH DETECTION (perform hough transform)

kernel = np.ones((5, 5), np.uint8) 
output = cv2.cvtColor(img, cv2.COLOR_GRAY2RGB)
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

# WRITE TO SVG (TODO: DATABASE)

f = open('path.svg', 'w+')
f.write('<svg width="400" height="400" xmlns="http://www.w3.org/2000/svg">')

f.write('<path d="M')
for line in lines:
    for x1,y1,x2,y2 in line:
        f.write('{0} {1} {2} {3}'.format(x1, y1, x2, y2))
        cv2.line(output,(x1,y1),(x2,y2),(0,255,0),1)
f.write('"/>')

f.write('</svg>')
f.close()

cv2.imwrite('output.png', output)
