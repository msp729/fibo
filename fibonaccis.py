def mvec(A, v):
    return (
        A[0][0] * v[0] + A[1][0] * v[1],
        A[0][1] * v[0] + A[1][1] * v[1],
    )


def mmul(A, B):
    return (mvec(A, mvec(B, (1, 0))), mvec(A, mvec(B, (0, 1))))


def mpow(B, n):
    R = ((1, 0), (0, 1))
    while n:
        if n % 2:
            R = mmul(R, B)
        n //= 2
        if n:
            B = mmul(B, B)
    return R


def flmmul(A, B):
    s2 = A[0] * B[0]
    s0 = A[1] * B[1]
    s1 = (A[0] + A[1]) * (B[0] + B[1]) - s0
    return (s1, s0 + s2)


def flmpow(B, n):
    R = (0, 1)
    while n:
        if n % 2:
            R = flmmul(R, B)
        n //= 2
        if n:
            B = flmmul(B, B)
    return R


# 34, 35
def recur(n):
    match n:
        case 0:
            return 0
        case 1:
            return 1
        case _:
            return recur(n - 1) + recur(n - 2)


#  59, 61-62
def g_rec(n):
    match n:
        case 0:
            return 0
        case 1:
            return 1
        case 2:
            return 1
        case 10:
            return 55
        case 20:
            return 6765
        case _:
            return g_rec(n - 3) + 2*g_rec(n - 2)


# 230k, 330k
def lin(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


# 2M, 2.4M
def matrix(n):
    step = ((1, 1), (1, 0))
    step = mpow(step, n)
    return step[0][1]


# 3M, 5M
def flmat(n):
    step = (1, 0)
    step = flmpow(step, n)
    return step[0]
