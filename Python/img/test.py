#%%
import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
from scipy.ndimage import gaussian_filter
import glob

#%%

out_dir = "/home/suzumura/Desktop/baito/out/"
root_dir = "/home/suzumura/Desktop/baito/png/"
png_list = glob.glob(root_dir + '*/*.png')

print(png_list)



#%%

img = Image.open(png_list[0])
print(np.array(img).shape)
plt.imshow(np.array(img))

#%%
# img = img.convert('L')

# im = np.array(img, 'f')

# im = gaussian_filter(im, 3)

# img = Image.fromarray(im)
# img = img.convert('L')

im = np.array(img)

print(im.max())

th = 244
im = (im > th) * 255
print(im.mean())
img = Image.fromarray(im, "L")

d = png_list[0].split("/")[-2]
out_file = png_list[0].replace(d+'/', '')
img.save(out_file)