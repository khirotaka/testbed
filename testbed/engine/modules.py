import numpy as np


def add_element(array: np.ndarray):
    for i in range(len(array)):
        array[i] = array[i] + 5

    return array
