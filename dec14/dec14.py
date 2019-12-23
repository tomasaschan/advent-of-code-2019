from collections import defaultdict
import sys
# recipie: str -> (str,int), [(str,int)]

def read():
  return sys.stdin.read()

def parse(input):
  return input.split("\n")

if __name__ == "__main__":
  input = read()

  print(parse(input))
