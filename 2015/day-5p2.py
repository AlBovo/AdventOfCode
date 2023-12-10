import sys
c = 0
for i in sys.stdin:
    f = False
    i = i.strip()
    for e in range(len(i)-1):
        if i.count(i[e:e+2]) > 1: 
            f = True
            break
    if not f:
        continue
    for e in range(len(i)-2):
        if i[e] == i[e+2]:
            c += 1
            break
print(c)