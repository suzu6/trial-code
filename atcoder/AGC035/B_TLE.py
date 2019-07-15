import sys

sys.setrecursionlimit(100000000)

def setTemp(i, j, arr):
    # print(i, j, arr)
    tmp.append([j, i])
    arr[i].discard(j)
    arr[j].discard(i)

    k = arr[j].pop()
    tmp.append([j, k])
    arr[k].discard(j)

    if len(arr[k]) == 1:
        setTemp(k, arr[k].pop(), arr)
    
    if len(arr[j]) == 1:
        setTemp(j, arr[j].pop(), arr)
    


def getOne(arr, M):
    t = len(tmp)
    if M == t:
        return
    
    # one
    for i, a in arr.items():
        if len(a) == 1:
            j = a.pop()
            setTemp(i, j, arr)

    
    t = len(tmp)
    if M == t:
        return
    
    arr = {i: v for i,v in arr.items() if len(v) > 0}
    
    # two
    for i, a in arr.items():
        if len(a) > 0:
            # print(i, a)
            j = a.pop()
            setTemp(i, j, arr)
            break
    return getOne(arr, M)




def main():
    N, M = list(map(int, input().split()))

    arr = {i+1 : set() for i in range(N+1)}
    for m in range(M):
        i, j = list(map(int, input().split()))
        arr[i].add(j)
        arr[j].add(i)

    if M % 2 == 1:
        print(-1)
        return

    # print(*arr, sep='\n')
    # print(arr[1])
    
    getOne(arr, M)
    # setTemp(1, 2, arr)
    for i, te in enumerate(tmp):
        print(*te, sep=" ")
            
    

if __name__ == "__main__":
    tmp = []
    main()
