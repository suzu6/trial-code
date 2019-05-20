import random


def babble_sort(arr):
    """ バブルソート """
    length = len(arr)

    for i in range(length):  # 3. 走査範囲を前からひとつ狭める
        for j in reversed(range(i+1, length)):
            # 1. 後ろから順に隣り合う要素を比較する。
            if arr[j-1] > arr[j]:
                # 2. 左が右の要素に比べ大きい場合交換する。
                arr[j-1], arr[j] = arr[j], arr[j-1]
    return arr


if __name__ == "__main__":
    # 長さ10のランダムな配列
    arr = list(range(10))
    src = random.sample(arr, len(arr))

    print(src)
    # [7, 2, 5, 3, 1, 6, 9, 4, 8, 0]

    dst = babble_sort(src)
    print(dst)
    # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
