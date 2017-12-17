#!/usr/bin/python3

from itertools import *

def genA(s = 679):
    while True:
        s = (s * 16807) % 2147483647
        yield s

def genB(s = 771):
    while True:
        s = (s * 48271) % 2147483647
        yield s


if __name__ == '__main__':

    y = zip(genA(), genB())
    s = islice(y, 40000000)
    f = lambda x: x[0] & 0xFFFF == x[1] & 0xFFFF
    r = sum(map(f, s))

    print("Result: {}".format(r))

