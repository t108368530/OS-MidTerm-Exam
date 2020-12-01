import time

import numpy

M=100
N=60

a = tuple(tuple(12.2 * i - 3.8 * j for j in range(1, N+1)) for i in range(1, M+1))
b = tuple(tuple(65.1 + 3.3 * i - 20.2 * j for j in range(1, M+1)) for i in range(1, N+1))
res = [[0 for j in range(1, M+1)] for i in range(1, M+1)]


# 驗證用 numpy.dot()
start = time.time()
numpy.dot(a, b)
end = time.time()
print("numpy dot:\n", numpy.dot(a, b)[99][30:50], "\n time:", end - start)
