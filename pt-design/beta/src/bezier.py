import numpy as np
import matplotlib.pyplot as plt

def cubic_bezier_sample(start, control1, control2, end):
    inputs = np.array([start, control1, control2, end])
    cubic_bezier_matrix = np.array([
        [-1,  3, -3,  1],
        [ 3, -6,  3,  0],
        [-3,  3,  0,  0],
        [ 1,  0,  0,  0]
    ])
    partial = cubic_bezier_matrix.dot(inputs)

    return (lambda t: np.array([t**3, t**2, t, 1]).dot(partial))

# == control points ==
start = np.array([30, 60])
control1 = np.array([20, 0])
control2 = np.array([40, 95])
end = np.array([100, 100])

# number of segments to generate
n_segments = 100
# get curve segment generator
curve = cubic_bezier_sample(start, control1, control2, end)
# get points on curve
points = np.array([curve(t) for t in np.linspace(0, 1, n_segments)])

# == plot ==
controls = np.array([start, control1, control2, end])
# segmented curve
plt.plot(points[:, 0], points[:, 1], '-')
# control points
plt.plot(controls[:,0], controls[:,1], 'o')
# misc lines
plt.plot([start[0], control1[0]], [start[1], control1[1]], '-', lw=1)
plt.plot([control2[0], end[0]], [control2[1], end[1]], '-', lw=1)

plt.show()


# 1 generate lines from bezier curves
# 2 compute a map of slopes for each point in the svg
# 3 find the longest contiguous lines of a common (or near common) slope
