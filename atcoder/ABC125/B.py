




def main():
    N = int(input())
    V = list(map(int, input().split()))
    C = list(map(int, input().split()))

    D = [V[i] - C[i] for i in range(N)]
    f = [d for d in D if d > 0]

    if len(f) == 0:
        print(0)
    else:
        print(sum(f))

if __name__ == "__main__":
    main()
