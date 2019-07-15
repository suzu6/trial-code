import sys
import math

sys.setrecursionlimit(10**6)



def put_que(n):
    global tmp, tree
    if n in tmp:
        return
    que.append(n)
    tmp.add(n)
    for i in tree[n]:
        put_que(i)


def search(n):
    global tree, que, answer

    if len(tree[n]) & 1:
        i = tree[n].pop()
        answer.append('{} {}'.format(i, n))
        tree[i].remove(n)

        j = tree[i].pop()
        answer.append('{} {}'.format(i, j))
        tree[j].remove(i)
    
    while len(tree[n]) > 0:
        i = tree[n].pop()
        answer.append('{} {}'.format(n, i))
        tree[i].remove(n)

def main():
    global tree, que, answer

    N, m = list(map(int, input().split()))
    graph = {i+1 : set() for i in range(N)}

    for _ in range(m):
        i, j = list(map(int, input().split()))
        graph[i].add(j)
        graph[j].add(i)

    if m % 2:
        print(-1)
        return

    # print(tree)
    tree = to_tree(graph)
    for n in reversed(que):
        search(n)
            
    print(*answer, sep="\n")


if __name__ == "__main__":
    tmp = set()
    que = []
    answer = []
    main()


"""
10 20
1 2
1 5
1 10
2 3
2 5
2 8
3 4
3 6
3 9
3 10
4 6
4 8
4 9
5 6
5 7
6 10
6 7
7 8
8 9
9 10
"""