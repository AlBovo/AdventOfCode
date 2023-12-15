import sys
t = sys.stdin.readline().strip().split(',')
sum = 0
for i in t:
    r = 0
    for e in i:
        r = ((r + ord(e)) * 17) % 256
    sum += r
print(sum)