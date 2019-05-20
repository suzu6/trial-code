import math
import random  # ランダムな配列の生成用


def quick_sort(arr):
    """ クイックソート """
    if len(arr) < 2:
        # 1. 走査する配列長が0か１の場合戻る。
        print(arr)
        return arr

    # 2. 走査する範囲の中央の要素をピボットとして選ぶ。
    pivot = math.floor(len(arr)/2)
    pivot_height = arr[pivot]
    left, right = 0, len(arr) - 1

    while(left < right):
        # 3. ピボットと比べ、大きい要素は左へ小さい要素は右へ交換する。
        while(arr[left] < pivot_height):
            # 左側の基準値より大きい位置まで移動
            left += 1
        while(arr[right] > pivot_height):
            # 右側の基準値より小さい位置まで移動
            right -= 1

        if (left < right):
            # leftがrightを超えていない場合、leftとrightを交換
            arr[left], arr[right] = arr[right], arr[left]
            print(arr, pivot_height)
        else:
            print()
            break
    # 4. 左右２つに配列を分割してこの関数を再帰的に繰り返す。
    arr[:left] = quick_sort(arr[:left])      # ピボットの左側をクイックソート
    arr[left+1:] = quick_sort(arr[left+1:])  # ピボットの右側をクイックソート
    return arr


if __name__ == "__main__":
    # 長さ10のランダムな配列
    arr = list(range(10))
    src = random.sample(arr, len(arr))

    print(src)
    # [7, 2, 5, 3, 1, 6, 9, 4, 8, 0]

    dst = quick_sort(src)
    print(dst)
    # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
