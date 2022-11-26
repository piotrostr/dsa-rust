#!/usr/bin/env python3

import matplotlib.pyplot as plt

permutations = [
    [1, 2, 3],
    [1, 3, 2],
    [2, 1, 3],
    [2, 3, 1],
    [3, 1, 2],
    [3, 2, 1],
]

print("press 'q' to go to the next permutation")

for permutation in permutations:
    plt.plot(permutation)
    plt.scatter(range(len(permutation)), permutation)
    plt.show()
