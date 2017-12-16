#!/usr/bin/python3
def partner(p):
    global ld
    sd = ''.join(ld)
    exchange([sd.find(p[0]), sd.find(p[1])])

def exchange(x):
    global ld
    ld[int(x[0])], ld[int(x[1])] = ld[int(x[1])], ld[int(x[0])]

def spin(n):
    global ld
    ld = ld[-n:] + ld[:-n]

def dance(m):
    global ld
    if m[0] == 's':
        c = int(m[1:])
        # print("Spin: {}".format(c))
        spin(c)
    elif m[0]== 'x':
        x = m[1:].split('/')
        # print("Exchange: {}".format(x))
        exchange(x)
    elif m[0] == 'p':
        p = m[1:].split('/')
        # print("Parner: {}".format(p))
        partner(p)

# omcdaflhnpjegkib
if __name__ == '__main__':
    d = "abcdefghijklmnop"
    ex = "abcde"

    ld = list(d)

    with open("p1.txt") as f:
        i = f.read().strip()

    moves = i.split(',')
    for m in moves:
        dance(m.strip())
        # print(ld)

    print("Result: {}".format(''.join(ld)))
