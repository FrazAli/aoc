#!/usr/bin/python

if __name__ == '__main__':
    fa = 16807
    fb = 48271
    
    sa = 679
    sb = 771

    d = 2147483647

    mask = int('00000000000000001111111111111111', 2)

    count = 0

    last = 40000000

    print("{:>12}\t{:>12}".format("--Gen. A--", "--Gen. B--"))
    for i in range(0, last):
        sa *= fa
        sb *= fb
        va = sa % d
        vb = sb % d

        if (va & mask) == (vb & mask):
            count += 1
            print("Count: {}".format(count))

        #print("{:>12}\t{:>12}".format(va, vb))
        #print("{0:>32b}\n{1:>32b}".format(va, vb))
        #print("{0:>32b}".format(mask))
        #print("Mask: {}".format(mask))
        #print("i: {}".format(i))
    
    print("Result: {}".format(count))
