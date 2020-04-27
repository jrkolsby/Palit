from PIL import Image, ImageFont, ImageDraw
import cv2
import math
import numpy as np
from shapely import geometry, ops
import sqlite3

# FONT RENDERING

SIZE = 216
HOUGH_THRESH = 8
HOUGH_MIN_LEN = 10
HOUGH_MAX_GAP = 2
MAX_LINES = 70

#CHARSET = "人攴/~彡厂二工田冂冖丿艸匚爪巛州儿冫山彳乙丶QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm,<.>/?;:'\"~`1234567890-=!@#$%^&*()_+[]{}\\|▗▖▄▝▐▞▟▘▚▌▙▀▜▛█━┃┅┇┉┋┏┓┗┛┣┫┳┻╋╏"
CHARSET = "人攴/~彡厂二工田冂冖丿艸匚爪巛州儿冫山彳乙丶QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm,<.>/?;:'\"~`1234567890-=!"

# Initialize unicode font

font = ImageFont.truetype("fonts/OsakaMono.ttf", SIZE) # Load font

# Initialize SQL DB

db = sqlite3.connect('lines.db')

# Initialize SVG to display results

svg = open('path.svg', 'w+')
svg.write('<svg width="{0}" height="{1}" xmlns="http://www.w3.org/2000/svg">'
        .format(SIZE, SIZE * len(CHARSET)))
svg.write('<style> .small { font: italic 5px sans-serif; } </style>')
svg_offset = 0 # We're gonna make a column of chars

for char in CHARSET:

    char_img = Image.new("L", (SIZE, SIZE), 0) #Initialize black bg
    draw = ImageDraw.Draw(char_img)
    draw.text((0, 0), char, font=font, fill=(255)) # Draw white char

    img = np.array(char_img) # Convert to numpy array

    # SKELETONIZATION and PATH DETECTION (perform hough transform)

    kernel = np.ones((9, 9), np.uint8) 
    # output = cv2.cvtColor(img, cv2.COLOR_GRAY2RGB)
    lines = None
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

    merged_line = ops.linemerge(multi_line)

    # WRITE TO DATABASE

    for line in merged_line:
        [[x1, y1], [x2, y2]] = line.coords

        dx = x2 - x1
        dy = y2 - y1

        theta = 90 if dx == 0 else (180 * math.atan(dy / dx)) / np.pi
        length = math.sqrt(dx**2 + dy**2)
        mid_x = x1 + (dx / 2)
        mid_y = y1 + (dy / 2)

        theta = int(theta)
        length = int(length)
        mid_x = int(mid_x)
        mid_y = int(mid_y)

        # db.execute(...)

        svg.write('<path stroke="red" stroke-width="1" d="M {0} {1} {2} {3} "/>'
                .format(int(x1), svg_offset + int(y1), 
                    int(x2), svg_offset + int(y2)))
        svg.write('<text class="small" x="{0}" y="{1}">{2} {3}</text>'
                .format(mid_x, svg_offset + mid_y, theta, length))

    svg_offset += SIZE

svg.write('"</svg>')
svg.close()
