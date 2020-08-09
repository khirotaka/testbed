import time
import numpy as np
from testbed._rust import show_items1d


a = np.array([i for i in range(1000)], dtype=np.float64)

start = time.time()
for i in range(len(a)):
    print(a[i])


py_speed = time.time() - start

start = time.time()
show_items1d(a)
rust_speed = time.time() - start

print("Python speed: {:.5f}".format(py_speed))
print("Rust speed:   {:.5f}".format(rust_speed))

# Python speed: 0.00248
# Rust speed:   0.00109
