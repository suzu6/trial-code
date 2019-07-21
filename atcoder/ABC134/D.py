import math

def main():
    N = int(input())
    A = [0] + list(map(int, input().split()))

    ra = [i+1 for i in range(N)]
    for i in reversed(ra):
        print(i)
    

if __name__ == "__main__":
    main()
