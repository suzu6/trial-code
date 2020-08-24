
def main():
    N = int(input())
    A = list(map(int, input().split()))
    
    m = A[0]
    S = 0

    for a in A:
        if m - a > 0:
            S += m - a
        if m < a:
            m = a
    
    print(S)


if __name__ == "__main__":
    main()