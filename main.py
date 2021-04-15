
from math import gcd


def gcdext(a, b):
    if b == 0:
        return (a, 1, 0)

    d, x, y = gcdext(b, a % b)
    x, y = y, x-(a//b)*y
    return (d, x, y)


m=20
d=13

(_,_,e)=gcdext(20, 13)
if e < 0:
    e+=m
print(f"e={e}")

print(pow())