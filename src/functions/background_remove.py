import cv2
import numpy as np
import sys


name = sys.argv[1] if len(sys.argv) >= 2 else "temp.png"
source = "./tmp/" + name

# Load the foreground input image
foreground = cv2.imread(source, cv2.IMREAD_GRAYSCALE)
rgb = foreground[0, 0]
avg = int(np.average(np.average(foreground, axis=0), axis=0))

foreground = cv2.bitwise_not(foreground)
th, im_th = cv2.threshold(foreground, 255 - (avg+ (255-rgb)), 255, cv2.THRESH_BINARY)
im_th = cv2.bitwise_not(im_th) if rgb < 128 else im_th
# im_th = cv2.GaussianBlur(im_th, (7, 7), 0)
im_floodfill = im_th.copy()

h, w = im_th.shape[:2]
mask = np.zeros((h+2, w+2), np.uint8)
cv2.floodFill(im_floodfill, mask, (0, 0), 255)

im_floodfill_inv = cv2.bitwise_not(im_floodfill)
im_out = im_th | im_floodfill_inv

foreground = cv2.imread(source)

b, g, r = cv2.split(foreground)
rgba = [b, g, r, im_out]
dst = cv2.merge(rgba, 4)
cv2.imwrite("./tmp/fixed/" + name, dst)
