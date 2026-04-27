from importlib.metadata import pass_none
from operator import and_

# x = 1
# y = 1
# while 1000000 > x:
#    print(x)
#    x = x + y
#    print(y)
#    y = y + x


x, y = 0, 1
while x < 100000: print(x, y); x += y; y += x