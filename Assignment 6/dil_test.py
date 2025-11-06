import numpy as np
import matplotlib.pyplot as plt
from scipy.ndimage import binary_dilation, binary_erosion

A = np.array(
    [
        [
            0,
            1,
            1,
            1,
            1,
            0,
            1,
        ],
        [
            1,
            0,
            1,
            0,
            1,
            1,
            0,
        ],
        [
            1,
            1,
            1,
            0,
            0,
            1,
            0,
        ],
        [
            1,
            0,
            1,
            1,
            0,
            0,
            0,
        ],
        [
            1,
            0,
            1,
            1,
            1,
            0,
            1,
        ],
        [
            0,
            1,
            1,
            0,
            0,
            1,
            1,
        ],
        [
            0,
            1,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    dtype=bool,
)

B = np.array([[1], [1], [1]], dtype=bool)

dilated_A = binary_dilation(A, structure=B)

# Print the results
print("Original matrix A:")
print(A.astype(int))
print("\nStructuring element B:")
print(B.astype(int))
print("\nDilated result:")
print(dilated_A.astype(int))

# Create visualization
fig, axes = plt.subplots(1, 3, figsize=(12, 4))

# Original image A
axes[0].imshow(A, cmap='gray', interpolation='nearest')
axes[0].set_title('Original Image A')
axes[0].grid(True, alpha=0.3)
axes[0].set_xticks(range(A.shape[1]))
axes[0].set_yticks(range(A.shape[0]))

# Add text annotations for A
for i in range(A.shape[0]):
    for j in range(A.shape[1]):
        axes[0].text(j, i, str(int(A[i, j])), ha='center', va='center',
                     color='red' if A[i, j] else 'blue', fontweight='bold')

# Structuring element B
axes[1].imshow(B, cmap='gray', interpolation='nearest')
axes[1].set_title('Structuring Element B')
axes[1].grid(True, alpha=0.3)
axes[1].set_xticks(range(B.shape[1]))
axes[1].set_yticks(range(B.shape[0]))

# Add text annotations for B
for i in range(B.shape[0]):
    for j in range(B.shape[1]):
        axes[1].text(j, i, str(int(B[i, j])), ha='center', va='center',
                     color='red' if B[i, j] else 'blue', fontweight='bold')

# Dilated result
axes[2].imshow(dilated_A, cmap='gray', interpolation='nearest')
axes[2].set_title('Dilated Image')
axes[2].grid(True, alpha=0.3)
axes[2].set_xticks(range(dilated_A.shape[1]))
axes[2].set_yticks(range(dilated_A.shape[0]))

# Add text annotations for dilated result
for i in range(dilated_A.shape[0]):
    for j in range(dilated_A.shape[1]):
        axes[2].text(j, i, str(int(dilated_A[i, j])), ha='center', va='center',
                     color='red' if dilated_A[i, j] else 'blue',
                     fontweight='bold')

plt.tight_layout()
plt.show()


A = np.array([ [0, 1, 1, 1, 1, 0, 1, ],
[1, 0, 1, 0, 1, 1, 0, ],
[1, 1, 1, 0, 0, 1, 0, ],
[1, 0, 1, 1, 0, 0, 0, ],
[1, 0, 1, 1, 1, 0, 1, ],
[0, 1, 1, 0, 0, 1, 1, ],
[0, 1, 0, 0, 0, 0, 0, ] ])


eroded_A = binary_erosion(A, structure=B)

# Print the results
print("Original matrix A:")
print(A.astype(int))
print("\nStructuring element B:")
print(B.astype(int))
print("\nEroded result:")
print(eroded_A.astype(int))

# Create visualization
fig, axes = plt.subplots(1, 3, figsize=(12, 4))

# Original image A
axes[0].imshow(A, cmap='gray', interpolation='nearest')
axes[0].set_title('Original Image A')
axes[0].grid(True, alpha=0.3)
axes[0].set_xticks(range(A.shape[1]))
axes[0].set_yticks(range(A.shape[0]))

# Add text annotations for A
for i in range(A.shape[0]):
    for j in range(A.shape[1]):
        axes[0].text(j, i, str(int(A[i, j])), ha='center', va='center',
                     color='red' if A[i, j] else 'blue', fontweight='bold')

# Structuring element B
axes[1].imshow(B, cmap='gray', interpolation='nearest')
axes[1].set_title('Structuring Element B')
axes[1].grid(True, alpha=0.3)
axes[1].set_xticks(range(B.shape[1]))
axes[1].set_yticks(range(B.shape[0]))

# Add text annotations for B
for i in range(B.shape[0]):
    for j in range(B.shape[1]):
        axes[1].text(j, i, str(int(B[i, j])), ha='center', va='center',
                     color='red' if B[i, j] else 'blue', fontweight='bold')

# Dilated result
axes[2].imshow(eroded_A, cmap='gray', interpolation='nearest')
axes[2].set_title('Eroded Image')
axes[2].grid(True, alpha=0.3)
axes[2].set_xticks(range(eroded_A.shape[1]))
axes[2].set_yticks(range(eroded_A.shape[0]))

# Add text annotations for eroded result
for i in range(eroded_A.shape[0]):
    for j in range(eroded_A.shape[1]):
        axes[2].text(j, i, str(int(eroded_A[i, j])), ha='center', va='center',
                     color='red' if eroded_A[i, j] else 'blue',
                     fontweight='bold')

plt.tight_layout()
plt.show()

