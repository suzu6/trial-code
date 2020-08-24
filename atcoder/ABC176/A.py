import math

def main():
    N, X, T = list(map(int, input().split()))
    
    print(math.ceil(N/X) * T)

if __name__ == "__main__":
    main()