import sys


N = int(input())
a = list(map(int, input().split()))

minimum = sys.maxsize

for i in range(1, N):
    # すぬけくんとアライグマのカードの総和の差の絶対値
    tmp = abs(sum(a[:i]) - sum(a[i:]))
    if tmp < minimum:
        minimum = tmp
print(minimum)
