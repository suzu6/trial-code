import A

def _output(S, L, di, path_out):
    with open(path_out, mode='w') as f:
        f.write(' '.join(list(map(str, [S, L]))) + '\n')
        for v, c in di.items():
            f.write(' '.join(list(map(str, [len(v.split()), c, *v.split()]))) + '\n')

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

def _input(path):
    s = ''
    with open(path) as f:
        s = f.readlines()
    
    N, K = list(map(int, s[0].split()))
    v = [[] for i in range(K)]
    c = [0 for i in range(K)]
    for i in range(K):
        tmp = list(map(int, s[i+1].split()))
        c[i] = tmp[1]
        v[i] = tmp[2:]

    return N, K, c, v



def main():
    path_in = './testcase.in'
    path_out = './result.out'

    N, K, c, v = _input(path_in)
    S, L, c, v = A.calc(N, K, c, v)
    di = similar_terms(c,v)
    _output(S, len(di), di, path_out)


if __name__ == "__main__":
    main()
