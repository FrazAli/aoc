#!/usr/bin/python3
if __name__ == '__main__':
    step = 377
    last = 2017
    cb = [0]
    r = 0

    pos = 0
    for i in range(1, last+1):
        pos = (pos + step) % i
        pos += 1
        cb.insert(pos, i)
        if i == last:
            r = cb[(pos + 1) % len(cb)]
        # print("{}({}), {}".format(pos, i, cb))

    print("Result: {}".format(r))
