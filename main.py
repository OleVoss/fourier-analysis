import cv2 as cv
import matplotlib.pyplot as plt
import numpy as np


img = cv.imread("circle.png")
figManager = plt.get_current_fig_manager()
figManager.window.showMaximized()
plt.imshow(img)
plt.show()
