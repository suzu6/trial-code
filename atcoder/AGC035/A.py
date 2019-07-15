import numpy as np




def main():
    N = int(input())
    V = list(map(int, input().split()))

    n = 0
    for i in V:
        n = i ^ n
    if n == 0:
        print('Yes')
    else:
        print('No')

if __name__ == "__main__":
    main()
