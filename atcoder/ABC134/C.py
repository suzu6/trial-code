import math

def main():
    N = int(input())
    A = []
    ma1 = 0
    ma2 = 0
    for i in range(N):
        a = int(input())
        A.append(a)
        if i == 1:
            if a >= A[0]:
                ma2 = 0
            else:
                ma2 = 1 
        if A[ma2] <=  a:
            if A[ma1] <= a:
                ma2 = ma1
                ma1 = i
            else:
                ma2 = i
    # print(ma, ma2)
    for i in range(N):
        if ma1 == i:
            print(A[ma2])
        else:
            print(A[ma1])
    

if __name__ == "__main__":
    main()
