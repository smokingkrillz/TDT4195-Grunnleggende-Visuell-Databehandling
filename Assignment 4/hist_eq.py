import numpy as np
import matplotlib.pyplot as plt

'''# Example: a small 3-bit image (values 0–7)
# You can replace this with your own data, e.g., image = np.random.randint(0, 8, (4, 4))
image = np.array([
    [4, 1, 1, 4, 5],
    [4, 4, 4, 0, 0],
    [4, 6, 4, 0, 3],
])

# Step 1: Calculate histogram
L = 8  # 3-bit image → 8 intensity levels
hist, bins = np.histogram(image.flatten(), bins=np.arange(L+1))
pdf = hist / image.size  # Probability density function

# Step 2: Compute cumulative distribution function (CDF)
cdf = np.cumsum(pdf)

# Step 3: Apply the histogram equalization formula
equalized_levels = np.round((L - 1) * cdf).astype(int)

# Step 4: Map original pixel values to new equalized values
equalized_image = equalized_levels[image]

# Step 5: Plot original vs equalized histogram
plt.figure(figsize=(10, 4))

plt.subplot(1, 2, 1)
plt.bar(range(L), hist, color='gray')
plt.title("Original Histogram")
plt.xlabel("Intensity Level")
plt.ylabel("Frequency")

plt.subplot(1, 2, 2)
plt.bar(range(L), np.histogram(equalized_image.flatten(), bins=np.arange(L+1))[0], color='gray')
plt.title("Equalized Histogram")
plt.xlabel("Intensity Level")

plt.tight_layout()
plt.show()

# Print results
print("Original image:\n", image)
print("\nHistogram:", hist)
print("PDF:", np.round(pdf, 3))
print("CDF:", np.round(cdf, 3))
print("Equalized mapping:", equalized_levels)
print("\nEqualized image:\n", equalized_image)
'''

from scipy.signal import correlate2d, convolve2d

I = np.array([[4, 1],
              [1, 4]])
K = np.array([[1, 0, -1],
              [2, 0, -2],
              [1, 0, -1]])

# Valid mode (no padding) → empty result
#print(correlate2d(I, K, mode='valid'))

# Same mode (with padding) → works
print(convolve2d(I, K, mode='same'))
