#!/usr/bin/python

def read_input(filename):
	data = []
	with open(filename) as f:
		while True:
			c = f.read(1)
			if not c:
				print("EOF")
				break

			if c.isdigit():
				data.append(int(c))

			# print("Read a character: %c" % c)
	return data

def parse_input(data, size):
    step = size / 2
    result = 0
    for i in range(0, size):
        if data[i] == data[(i + step) % size]:
            result += data[i]

    return result

if __name__ == '__main__':
    data = read_input("p2-input.txt")
    # print("data: %i" % len(data))
    result = parse_input(data, len(data))
    print("Result: %i" % result)
