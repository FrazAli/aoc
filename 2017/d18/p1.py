#!/usr/bin/python3

from itertools import *
from string import ascii_lowercase
from sys import setrecursionlimit

def getv(r, n):
    if str(n).startswith('-') or str(n).isdigit():
        return int(n)
    else:
        return getv(r, r.get(n, 0))

def parse(i, r):
    i,d,*s = i.split(' ')

    if i == "snd":
        r['0'] = getv(r, d)
    elif i == "set":
        r[d] = getv(r, s[0])
    elif i == "add":
        r[d] = getv(r, d) + getv(r, s[0])
    elif i == "mul":
        r[d] = getv(r, d) * getv(r, s[0])
    elif i == "mod":
        r[d] = getv(r, d) % getv(r, s[0])
    elif i == "rcv":
        f = getv(r, d)
        if f > 0:
            r['1'] = r['0']
    elif i == "jgz":
        d = getv(r, d)
        if d > 0:
            return getv(r, s[0])

    return 0

def run(s, i, regs):
    if s == len(i) - 1 or i[s] == '':
        return

    r = parse(i[s], regs)
    if regs['1'] != 0:
        print("Result: {}".format(regs['1']))
        return

    s = s+1 if r == 0 else s+r
    run(s, i, regs)

if __name__ == "__main__":
    setrecursionlimit(1500)
    regs = {}
    regs['0'] = 0 # Sound register
    regs['1'] = 0 # Recovery register
    with open("p1.txt") as f:
        ins = f.read().split('\n')
        run(0, ins, regs)

