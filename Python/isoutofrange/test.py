def recal(arr, index):
    if len(index) == 0:
        return
    else:
        recal(arr[index[0]], index[1:])


def is_out_of_range(arr, *index):
    try:
        recal(arr, index)
    except IndexError:
        return True
    else:
        return False


if __name__ == "__main__":
    array = [[i for i in range(10)] for j in range(10)]
    print(*array, sep="\n")

    print(is_out_of_range(array, 0, 0))
    print(is_out_of_range(array, 1, 10))
    print(is_out_of_range(array, 2, 0))
