import math

A, B, T = list(map(int, input().split()))

print(math.floor(T / A) * B)
