#!/usr/bin/python3
if __name__ == '__main__':
    step = 377
    last = 50000000
    cb = [0]
    r = 0

    pos = 0 
    for i in range(1, last+1):
        pos = (pos + step) % i
        if pos == 0:
            r = i
            print(r)
        pos += 1
    
    print("Result: {}".format(r))
