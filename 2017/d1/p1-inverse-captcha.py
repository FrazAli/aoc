#!/usr/bin/python

def read_input(filename):
	with open(filename) as f:
		while True:
			c = f.read(1)
			if not c:
				print("EOF")
				break
			print("Read a character: %c" % c)

if __name__ == '__main__':
	read_input("p1-input.txt")
