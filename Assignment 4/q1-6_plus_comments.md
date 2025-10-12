1. Which of the following best describes the process of sampling?


The process of converting a continuous signal to a discrete one by reading values at a set time interval.

2.  What is quantization in the context of signal processing?



The process of converting continuous amplitudes from sampled values by rounding to one of the possible discrete values.



3. When looking at an image histogram, which characteristic indicates high contrast?

The histogram is evenly distributed across the entire intensity spectrum.

4. What is the effect of applying a log transform to an image with a large variance in pixel intensities on its dynamic range?


The dynamic range decreases.
5. intensities =       [0, 1, 2, 3, 4, 5, 6, 7]
intensity_count =   [3, 2, 0, 1, 7, 1, 1, 0]
pdf =               [0.200, 0.133,0.0 ,0.067,0.467, 0.067, 0.067, 0.0] # Probability Density Function
cdf =               [0.2, 0.333, 0.333, 0.4, 0.867,0.933,1.0,1.0] # Cumulative Distribution Function
new_intensities =   [1, 2, 2, 2, 6, 6, 7, 7] # Round down any resulting pixel intensities that are not integers (use the floor operator)

M_equalized =     [[6,2, 2, 6, 6],
                [6,6,6, 1, 1],
                [6, 7, 6, 1, 2]]

6. B = [[6, -9],
    [9, -6]]

7. B = [[-6, 9],
    [-9, 6]]

question 14-17 is all done in one .py file