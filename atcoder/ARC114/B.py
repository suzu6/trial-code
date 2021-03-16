import math


def main():
    N = int(input())
    X = list(map(int, input().split()))

    prims = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]

    result = 1
    aaa = set()
    for p in prims:
        print(X)
        new = []
        for x in X:
            if x % p == 0:
                aaa.add(p)
                continue
            new.append(x)
        X = new

    print(aaa)
    for a in aaa:
        result = result * a

    print(result)


if __name__ == "__main__":
    main()
