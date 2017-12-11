#!/usr/bin/python

def read_input(filename):
	last = None
	first = None
	result = 0
	with open(filename) as f:
		while True:
			c = f.read(1)
			if not c:
				if first == last:
					result += int(last)
				print("f: %s" % first)
				print("l: %s" % last)
				print("EOF")
				break

			if first is None:
				first = c

			if c == last:
				result += int(c)
				print("Result: %i" % result)

			if c.isdigit():
				last = c

			print("Read a character: %c" % c)
	return result

if __name__ == '__main__':
	r = read_input("p1-input.txt")
	print("Result: %d" % r)
