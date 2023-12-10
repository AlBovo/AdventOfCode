from multiset import *
import sys, re
m = [i.strip().split('|') for i in sys.stdin]

for i in m:
    i[0] = i[0][i[0].index(":")+2:].strip()
    i[1] = i[1].strip()
    
    i[0] = [int(e) for e in re.findall(r'\d+', i[0])]
    i[1] = [int(e) for e in re.findall(r'\d+', i[1])]
    
d = Multiset({i for i in range(len(m))})

sum = 0
for i in range(len(m)):
    t = d[i]
    # print(d, t)
    d.remove(i)
    c = i + 1
    
    for e in m[i][1]:
        if c >= len(m):
            break
        if e in m[i][0]:
            d.add(c, t)
            c += 1

    sum += t
print(sum)