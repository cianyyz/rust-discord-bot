import cv2
import numpy as np
import sys

#  Todo: Clear ./tmp/ and ./tmp/fixed
#  Todo: Make it output many images, because it does not cover all use cases

#  Prerequisite: Have an image in the ./tmp/ folder.
#
#  If you have an image in ./tmp/ called example.png use this command
#  python src/commands/background_remove.py example.png

name = sys.argv[1] if len(sys.argv) >= 2 else "temp.png"
source = "./tmp/" + name

# Load the foreground input image
foreground = cv2.imread(source, cv2.IMREAD_GRAYSCALE)

#  Checks if it found a file, for some reaosn no exception is throw in cv2.imread
if foreground is None:
    raise Exception("No file found")

rgb = foreground[0, 0] # Gets grayscale value of first pixel
avg = int(np.average(np.average(foreground, axis=0), axis=0)) # Gets average grayscale value ( Average of the average value of each row)

foreground = cv2.bitwise_not(foreground) # Inverts image
threshold = 255 - (avg+ (255-rgb)) # Most general use case
th, im_th = cv2.threshold(foreground, threshold, 255, cv2.THRESH_BINARY) # Makes everything above threshold white and rest black
im_th = cv2.bitwise_not(im_th) if rgb < 128 else im_th # Inverts image if its more black on the edge

im_floodfill = im_th.copy() # Copies image

h, w = im_th.shape[:2] #gets height and width
mask = np.zeros((h+2, w+2), np.uint8) # Adds 2 pixels around the border to ensure the floodfill works properly
cv2.floodFill(im_floodfill, mask, (0, 0), 255) # Acts similar to bucket on paint

im_floodfill_inv = cv2.bitwise_not(im_floodfill) # Inverts floodfileld image
im_out = im_th | im_floodfill_inv # Bitwise OR  ( Adding the original threshold whites and the inverted floodfill whites)

foreground = cv2.imread(source) # Retreieves the image in color

b, g, r = cv2.split(foreground) # Gets rgb of the iamge
rgba = [b, g, r, im_out] # uses the alpha of the calculated floodfileld mask
dst = cv2.merge(rgba, 4) # Wraps up the images
cv2.imwrite("./tmp/fixed/" + name, dst) # Writes to fixed folder
