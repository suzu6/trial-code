import math


def main():
    N = int(input())
    A = list(map(int, input().split()))

    B = list(map(abs, A))
    C = [a for a in A if a < 0]
    if len(C) % 2 == 0:
        print(sum(B))
    else:
        print(sum(B) - min(B)*2)
    

if __name__ == "__main__":
    main()
