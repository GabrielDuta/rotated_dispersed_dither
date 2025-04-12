import numpy as np
import matplotlib.pyplot as plt
from PIL import Image

# Load the dithered image in grayscale
image = Image.open("output16_1.png").convert("L")  # 'L' = grayscale
img_array = np.array(image)

# Apply 2D FFT
f_transform = np.fft.fft2(img_array)
f_shifted = np.fft.fftshift(f_transform)  # Shift zero freq to center
magnitude_spectrum = 20 * np.log(np.abs(f_shifted) + 1)  # Log scale for visibility

# Display results
plt.figure(figsize=(12, 6))

plt.subplot(1, 2, 1)
plt.imshow(img_array, cmap='gray')
plt.title("Dithered Image")
plt.axis("off")

plt.subplot(1, 2, 2)
plt.imshow(magnitude_spectrum, cmap='gray')
plt.title("Frequency Domain (FFT)")
plt.axis("off")

plt.tight_layout()
plt.show()

def radial_profile(data):
    y, x = np.indices((data.shape))
    center = np.array([(x.max()-x.min())/2.0, (y.max()-y.min())/2.0])
    r = np.hypot(x - center[0], y - center[1])
    r = r.astype(np.int32)

    tbin = np.bincount(r.ravel(), data.ravel())
    nr = np.bincount(r.ravel())
    radial_profile = tbin / (nr + 1e-8)
    return radial_profile

# Example usage
profile = radial_profile(np.abs(f_shifted))
plt.plot(profile)
plt.title("Radial Frequency Spectrum")
plt.xlabel("Frequency Radius")
plt.ylabel("Magnitude")
plt.show()
