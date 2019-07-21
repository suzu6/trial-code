import math

def main():
    N, D = list(map(int, input().split()))

    a = math.ceil(N / (2 * D + 1))
    print(a)

if __name__ == "__main__":
    main()
