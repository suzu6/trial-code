import math


def _output(S, L, di):
    print(S, L)
    for v, c in di.items():
        print(len(v.split()), c, *v.split())


def similar_terms(c, v):
    # 同類項をまとめる
    di = {}
    for c, v in zip(c, v):
        if len(v) is 0:
            di[' '] = c
            continue
        v = ' '.join(map(str, v))
        if di.get(v) is None:
            di[v] = c
        else:
            di[v] += c
    return di


def _input():
    N, K = list(map(int, input().split()))
    v = [[] for i in range(K)]
    c = [0 for i in range(K)]
    for i in range(K):
        tmp = list(map(int, input().split()))
        c[i] = tmp[1]
        v[i] = tmp[2:]

    return N, K, c, v


def minus(c, v, v_res, c_res, N):
    N += 1
    for x in v:
        v_res.append([x, N])
        c_res.append(c)
    v_res.append([N])
    c_res.append(-1 * c * (len(v) - 1))
    return v_res, c_res, N


def s2(v_i):
    arr = []
    for i in range(len(v_i)-1):
        for j in range(i+1, len(v_i)):
            arr.append([v_i[i], v_i[j]])
    return arr


def d3(c, v_i, v_res, c_res, N):
    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-1*c)
    v_res.append([N])
    c_res.append(c)

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def d4(c, v_i, v_res, c_res, N):
    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-2*c)
    v_res.append([N])
    c_res.append(c*3)

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def d5(c, v_i, v_res, c_res, N):
    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-2*c)
    v_res.append([N])
    c_res.append(c * 3)

    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-1*c)
    v_res.append([N])
    c_res.append(c * 3)

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def d6(c, v_i, v_res, c_res, N):
    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-2*c)
    v_res.append([N])
    c_res.append(c * 3)

    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-2*c)
    v_res.append([N])
    c_res.append(c * 7)

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def odd(c, v_i, v_res, c_res, N):
    d = len(v_i)
    nd = math.floor((d-1)/2)
    r = [i+1 for i in range(nd - 1)]
    for i in r:
        N += 1
        for x in v_i:
            v_res.append([x, N])
            c_res.append(-2*c)
        v_res.append([N])
        c_res.append(c * (4 * i - 1))

    N += 1
    for x in v_i:
        v_res.append([x, N])
        c_res.append(-1*c)
    v_res.append([N])
    c_res.append(c * (2 * nd - 1))

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def even(c, v_i, v_res, c_res, N):
    d = len(v_i)
    nd = math.floor((d-1)/2)
    r = [i+1 for i in range(nd)]
    for i in r:
        N += 1
        for x in v_i:
            v_res.append([x, N])
            c_res.append(-2*c)
        v_res.append([N])
        c_res.append(c * (4 * i - 1))

    s = s2(v_i)
    v_res.extend(s)
    c_res.extend([c] * len(s))
    return v_res, c_res, N


def calc(N, K, c, v):
    v_res = []
    c_res = []

    const = 0

    for i in range(K):
        d = len(v[i])
        if d == 0:
            const += c[i]
        elif d <= 2:
            v_res.append(v[i])
            c_res.append(c[i])
        elif c[i] < 0:
            v_res, c_res, N = minus(c[i], v[i], v_res, c_res, N)
        elif d % 2 == 0 and c[i] > 2:
            v_res, c_res, N = even(c[i], v[i], v_res, c_res, N)
        elif d % 2 == 1 and c[i] > 2:
            v_res, c_res, N = odd(c[i], v[i], v_res, c_res, N)
        else:
            const += c[i]

    if const != 0:
        v_res.append([])
        c_res.append(const)
    return N, len(v_res), c_res, v_res


def main():
    N, K, c, v = _input()
    S, L, c, v = calc(N, K, c, v)
    di = similar_terms(c, v)
    _output(S, len(di), di)


if __name__ == "__main__":
    main()
