import time
import numpy as np
from testbed.testbed import sliding_window


x = np.random.randn(100000, 5)


s = time.time()
rustout = sliding_window(x, 500, 1)
print("=" * 50)
print(time.time() - s)
print(rustout.shape)


def sw(array, ws, over):
    sl = len(array)
    return [array[i:i+ws] for i in range(0, sl-ws, over)]


print("=" * 50)
s = time.time()
tmp = sw(x, 500, 1)
tmp = np.stack(tmp, 0)
print(time.time() - s)
print(tmp.shape)
