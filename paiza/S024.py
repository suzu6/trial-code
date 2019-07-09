import copy


def judge(x, n):
    if x > n:
        return 0
    else:
        return -1


def relabel(area, n, H):

    for y in range(H):
        area[y] = list(map(lambda x: judge(x, n), area[y]))
    return area


def isOutOfRange(area, x, y):
    try:
        area[x][y]
    except IndexError:
        return True
    else:
        return False


def countIslands(area, H, W):
    label = 1
    lookup = {}
    for y in range(H):
        for x in range(W):
            if area[y][x] != 0:
                continue
            arr = [-1, -1]
            if x > 0:
                arr[0] = area[y][x-1]
            if y > 0:
                arr[1] = area[y-1][x]
            arr = [i for i in arr if i > 0]
            if len(arr) == 0:
                area[y][x] = label
                lookup[label] = label
                label += 1
                continue
            area[y][x] = min(*arr, label)
            lookup[max(arr)] = min(*arr, label, lookup[max(arr)])

    count = 0
    # print(lookup)
    for k, v in lookup.items():
        if k == v:
            count += 1
    return count


def main():
    H, W, N = list(map(int, input().split()))
    area = []
    all = [0]
    for i in range(H):
        area.append(list(map(int, input().split())))
        all.extend(area[i])
    all = set(all)
    # print(H, W, N, all)
    # print(*area, sep="\n")
    for i in all:
        a = relabel(copy.deepcopy(area), i, H)
        count = countIslands(a, H, W)
        # print(*a, sep="\n")
        # print(i, count)
        if count == N:
            # print(*a, sep="\n")
            # print(count)
            print(i)
            break


if __name__ == "__main__":
    main()
