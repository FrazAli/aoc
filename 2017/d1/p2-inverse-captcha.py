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

			print("Read a character: %c" % c)
	return data

def parse_input(data, size):
    return 0

if __name__ == '__main__':
    data = read_input("p1-input.txt")
    print("data: %i" % len(data))
    result = parse_input(data, len(data))
    print("Result: %i" % result)
