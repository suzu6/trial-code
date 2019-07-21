from bisect import bisect


def main():
    N = int(input())
    A = [-int(input()) for _ in range(N)]
    L = [A[0]]
    for a in A[1:]:
        if a >= L[-1]:
            L.append(a)
        else:
            L[bisect(L, a)] = a
    print(len(L))


if __name__ == "__main__":
    main()
