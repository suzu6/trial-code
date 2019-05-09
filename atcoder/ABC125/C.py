
def gcd(a, b):
    while(b):
        a, b = b, a % b
    return a


def main():
    N = int(input())
    A = list(map(int, input().split()))

    index = list(range(N))
    forward = index.copy()
    forward[0] = 0
    for i in index[:-1]:
        forward[i+1] = gcd(A[i], forward[i])

    backward = index.copy()
    backward.append(0)
    for i in reversed(index):
        backward[i] = gcd(A[i], backward[i+1])
    print(forward)
    print(backward)
    M = [gcd(f, b) for f, b in zip(forward, backward[1:])]
    print(M)
    print(max(M))


if __name__ == "__main__":
    main()
