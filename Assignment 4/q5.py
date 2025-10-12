from altair import Step
import numpy as np
import matplotlib.pyplot as plt


image = np.array([
    [4, 1, 1, 4, 5],
    [4, 4, 4, 0, 0],
    [4, 6, 4, 0, 3],
])


L = 8  # 3-bit image → 8 intensity levels
hist, bins = np.histogram(image.flatten(), bins=np.arange(L+1))
pdf = hist / image.size  # Probability density function

cdf = np.cumsum(pdf)

equalized_levels = np.round((L - 1) * cdf).astype(int)


equalized_image = equalized_levels[image]


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

print("Original image:\n", image)
print("\nHistogram:", hist)
print("PDF:", np.round(pdf, 3))
print("CDF:", np.round(cdf, 3))
print("Equalized mapping:", equalized_levels)
print("\nEqualized image:\n", equalized_image)


