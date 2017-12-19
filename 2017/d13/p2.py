#!/usr/bin/python3

if __name__ == '__main__':
    with open("p1.txt") as f:
        d = dict([line.replace(' ','').replace('\n','').split(':') for line in f])
        d = {int(k):int(v) for k,v in d.items()}
        r = 0
        while True:
            c = False
            for p, s in d.items():
                if (p + r) % ((s - 1) * 2) == 0:
                    c = True
                    break
            if c:
                r += 1
            else:
                break

        print("Result: {}".format(r))

