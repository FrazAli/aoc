#!/usr/bin/python
def parse_line(line):
    rec = dict()
    line = line.split('\n')
    tokens = line[0].split('<->')
    key = int(tokens[0].strip())
    vals = [int(t.strip()) for t in tokens[1].split(',')]
    rec[key] = vals
    print rec
    return rec

def read_input(filename):
    data = {}
    with open(filename) as f:
        while True:
            line = f.readline()
            if line == '':
                print("EOF")
                break

            # print("Read a line: %s" % line)
            data.update(parse_line(line))
    return data

def search_programs(data, p):
    global count
    global pgms
    if p not in pgms:
        count += 1
        pgms.append(p)
    else:
        return

    for i in data[p]:
        search_programs(data, i)

if __name__ == '__main__':
    count = 0
    groups = 0
    d = read_input("p2-input.txt")
    while d:
        pgms = []
        groups += 1
        search_programs(d, next(iter(d)))
        print pgms
        for i in pgms:
             del d[i]
    print("Result: %i" % groups)
