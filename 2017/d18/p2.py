#!/usr/bin/python3

from itertools import *
from string import ascii_lowercase
from collections import deque

def getv(r, n):
    if str(n).startswith('-') or str(n).isdigit():
        return int(n)
    else:
        return getv(r, r.get(n, 0))

def run0(ins, r, sq, rq):
    ip = 0
    while ip < len(ins) or ins[ip] != '':
        i,d,*s = ins[ip].split(' ')

        if i == "snd":
            r['s'] += 1
            sq.append(getv(r, d))
        elif i == "set":
            r[d] = getv(r, s[0])
        elif i == "add":
            r[d] = getv(r, d) + getv(r, s[0])
        elif i == "mul":
            r[d] = getv(r, d) * getv(r, s[0])
        elif i == "mod":
            r[d] = getv(r, d) % getv(r, s[0])
        elif i == "rcv":
            if rq:
                r[d] = rq.popleft()
                r['r'] += 1
            else:
                yield None
                continue
        elif i == "jgz":
            d = getv(r, d)
            if d > 0:
                ip += getv(r, s[0])
                continue

        ip += 1
        yield ip

def run1(ins, r, sq, rq):
    ip = 0
    while ip < len(ins) or ins[ip] != '':
        i,d,*s = ins[ip].split(' ')

        if i == "snd":
            r['s'] += 1
            sq.append(getv(r, d))
        elif i == "set":
            r[d] = getv(r, s[0])
        elif i == "add":
            r[d] = getv(r, d) + getv(r, s[0])
        elif i == "mul":
            r[d] = getv(r, d) * getv(r, s[0])
        elif i == "mod":
            r[d] = getv(r, d) % getv(r, s[0])
        elif i == "rcv":
            if rq:
                r[d] = rq.popleft()
                r['r'] += 1
            else:
                yield None
                continue
        elif i == "jgz":
            d = getv(r, d)
            if d > 0:
                ip += getv(r, s[0])
                continue

        ip += 1
        yield ip

if __name__ == "__main__":

    with open("p1.txt") as f:
        ins = f.read().split('\n')

    q0 = deque()
    q1 = deque()

    regs0 = {}
    regs1 = {}

    regs0['s'] = regs0['r'] = 0
    regs1['s'] = regs1['r'] = 0

    regs0['p'] = regs0['pid'] = 0
    regs1['p'] = regs1['pid'] = 1

    it0 = run0(ins, regs0, q1, q0)
    it1 = run1(ins, regs1, q0, q1)

    while True:
        while next(it0) is not None:
            continue

        while next(it1) is not None:
            continue

        if len(q1) > 0:
            it0.send(None)
            continue

        if len(q0) > 0:
            it1.send(None)
            continue

        if len(q0) == 0 and len(q1) == 0:
            print("Result: {}".format(regs1['s']))
            break

