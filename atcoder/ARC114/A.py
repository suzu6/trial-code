import math
import random
import itertools


def test(X, prism):
    
    aaa = set()
    for p in prism:
        # print(X)
        new = []
        for x in X:
            if x % p == 0:
                aaa.add(p)
                continue
            new.append(x)
        X = new
    return aaa


def main():
    N = int(input())
    X_o = list(map(int, input().split()))
    X = X_o
    prims = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]

    aaa = test(X, prims)
    ll = list(aaa)
    aaa = test(X, sorted(ll, reverse=True))
    ll = list(aaa)

    all = list(itertools.permutations(ll, len(ll)))
    print(all)

    r_min = 1000000000000000000000000000000000
    result = 1
    for al in all:
        aaa = test(X, al)
        ll = list(aaa)
        for a in ll:
            result = result * a
        if result < r_min:
            r_min = result

    print(r_min)


if __name__ == "__main__":
    main()
