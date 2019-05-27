

def swap(src, index):
    dst = [0 for i in range(len(src))]
    for (s, i) in zip(src, index):
        dst[i-1] = s
    return dst


def recal(src, P, min):
    # print(src)
    # print(P)
    for i in range(len(P)):
        d = P[:]
        next = swap(src, P[i])
        if next < min:
            min = next
        d.pop(i)
        min = recal(next, d, min)
    return min


def main():
    N = int(input())
    S = list(map(int, input().split()))
    min = S
    M = int(input())
    P = []
    for _ in range(M):
        P.append(list(map(int, input().split())))
    # print(S)
    # print(P)
    for i in range(M):
        min = recal(S, P, min)
    print(*min)


if __name__ == "__main__":
    main()
