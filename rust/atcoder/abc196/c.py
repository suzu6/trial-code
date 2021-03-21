import math

def main():
    S = input().strip()
    keta = len(S)

    if keta == 1:
        print(0)
        return

    cmax = 0
    if keta % 2 == 1:
        keta = (keta - 1)/2
        cmax = int(pow(10, keta))
        print(cmax - 1)
        return

    keta = int((keta)/2)
    cmax = math.floor(int(S) / pow(10, keta))
    if keta != len(str(cmax-1)):
      print(cmax-1)
      return
    sita = int(S) % pow(10, keta)
    if sita < cmax:
      cmax = cmax - 1
    print(cmax)


if __name__ == '__main__':
    main()
