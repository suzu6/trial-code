import math


def get_sieve_of_eratosthenes(n):
    if not isinstance(n, int):
        raise TypeError('n is int type.')
    if n < 2:
        raise ValueError('n is more than 2')
    prime = []
    limit = math.sqrt(n)
    data = [i + 1 for i in range(1, n)]
    while True:
        p = data[0]
        if limit <= p:
            return prime + data
        prime.append(p)
        data = [e for e in data if e % p != 0]


def dev(A, n):
    B = [int(a / n) for a in A if a % n == 0]
    return B, len(A) - len(B)




def main():
    N = int(input())
    A = list(map(int, input().split()))

    A.sort()
    _max = A[1]**0.5 + 1
    prime_list = get_sieve_of_eratosthenes(math.floor(_max))
    yaku = 1
    i = -1
    while True:
        i += 1
        if len(prime_list) < i +1 :
            break
        if len(A) == 0:
            break
        B, d = dev(A, prime_list[i])
        # print(A, prime_list[i], B, d)
        if d > 1:
            continue
        if N - len(B) > 1:
            continue
        yaku *= prime_list[i]
        A = B
        # print(yaku, prime_list[i])
        i -= 1
    print(yaku)
    

if __name__ == "__main__":
    main()
