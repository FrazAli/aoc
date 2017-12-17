#!/usr/bin/python3

from itertools import *

def genA(s = 679):
    #s = 65
    while True:
        s = (s * 16807) % 2147483647
        if s % 4 == 0:
            yield s
def genB(s = 771):
    #s = 8921
    while True:
        s = (s * 48271) % 2147483647
        if s % 8 == 0:
            yield s


if __name__ == '__main__':

    y = zip(genA(), genB())
    s = islice(y, 5000000)
    # print("{}\n".format(list(s)))
    f = lambda x: x[0] & 0xFFFF == x[1] & 0xFFFF
    r = sum(map(f, s))

    print("Result: {}".format(r))

