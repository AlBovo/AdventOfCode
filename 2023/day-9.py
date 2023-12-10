import sys
m = [i.strip().split() for i in sys.stdin]

for i in range(len(m)):
    for e in range(len(m[i])):
        m[i][e] = int(m[i][e])

t = 0
for i in m:
    l = [i, []]
    pos = 0
    
    while True:
        for e in range(len(l[pos])-1):
            l[pos+1].append(l[pos][e+1] - l[pos][e])
        pos += 1
        if any([i != 0 for i in l[pos]]):
            l.append([])
        else:
            pos -= 1
            l = l[:-1]
            break
    l[pos].append(l[pos][-1])
    pos -= 1
    while pos >= 0:
        l[pos].append(l[pos+1][-1] + l[pos][-1])
        pos -= 1
    t += l[0][-1]
print(t)