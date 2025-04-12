from skimage.metrics import structural_similarity as ssim
from skimage.io import imread
from skimage import img_as_float
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import os

def mse(imageA, imageB):
    return np.mean((imageA - imageB) ** 2)

def psnr(imageA, imageB):
    mse_val = mse(imageA, imageB)
    if mse_val == 0:
        return float('inf')
    PIXEL_MAX = 1.0
    return 20 * np.log10(PIXEL_MAX / np.sqrt(mse_val))

def load_grayscale_image(path):
    img = imread(path)

    if img.ndim == 3:
        # Check if it's a gray RGB (R == G == B)
        if np.allclose(img[..., 0], img[..., 1]) and np.allclose(img[..., 0], img[..., 2]):
            img = img[..., 0]  # Use one channel since all are the same
        else:
            raise ValueError(f"Image {path} is RGB or color, not grayscale.")

    img = img_as_float(img)  # Converts to float in [0, 1]
    return img

from os import listdir

def evaluate_images():
    results = []

    original_path = "Screenshot from 2025-04-03 15-43-23.png"
    original = load_grayscale_image(original_path)
    path = os.path.abspath(".")
    image_list = os.listdir("./results")
    image_list = [os.path.join(path, "results", f) for f in image_list if f.endswith(('.png', '.jpg', '.jpeg'))]

    for dithered_path in image_list:
        dithered = load_grayscale_image(dithered_path)

        # Ensure dimensions match
        if original.shape != dithered.shape:
            raise ValueError(f"Image size mismatch: {original_path} vs {dithered_path}")

        mse_val = mse(original, dithered)
        psnr_val = psnr(original, dithered)
        ssim_val = ssim(original, dithered, data_range=1.0)

        results.append({
            "Dithered": os.path.basename(dithered_path),
            "MSE": mse_val,
            "PSNR": psnr_val,
            "SSIM": ssim_val
        })

    # Sort results by Dithered image name
    results.sort(key=lambda x: x["MSE"])

    # Show results
    df = pd.DataFrame(results)
    print("\nDithering Evaluation Results:")
    print(df)

    '''
    # Plot
    df.plot(x="Dithered", y=["MSE", "PSNR", "SSIM"], kind="bar", figsize=(12, 6), title="Dithering Quality Metrics")
    plt.grid(True)
    plt.tight_layout()
    plt.show()
    '''

# Example usage:
# image_pairs = [("original_bw1.png", "dithered_bw1.png"), ("original_bw2.png", "dithered_bw2.png")]
# evaluate_images(image_pairs)

evaluate_images()