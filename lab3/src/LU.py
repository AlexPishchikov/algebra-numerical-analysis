A = [
    [1.0, 0.0, 3.0],
    [-4.0, 1.0, 333.0],
    [6.0, 99.0, 2.0]
]

f = [11.0, 2.0, 3.0]


L = [[0.0 for _ in range(len(A))] for _ in range(len(A))]
U = [[0.0 for _ in range(len(A))] for _ in range(len(A))]

for i in range(len(A)):
    for j in range(i, len(A)):
        U[i][j] = A[i][j] - sum([L[i][k] * U[k][j] for k in range(i)])
    for j in range(i, len(A)):
        L[j][i] = (A[j][i] - sum([L[j][k] * U[k][i] for k in range(i)])) / U[i][i]

y = [0.0 for _ in range(len(A))]
for i in range(len(A)):
    y[i] = f[i] - sum([L[i][k] * y[k] for k in range(i)])

x = [0.0 for _ in range(len(A))]
for i in range(len(A) - 1, -1, -1):
    x[i] = (y[i] - sum([U[i][k] * x[k] for k in range(i + 1, len(A))])) / U[i][i]

for row in L:
    print(row)
print()
for row in U:
    print(row)
print()

print(x)
