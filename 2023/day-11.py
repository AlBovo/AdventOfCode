import sys
r = [i.strip() for i in sys.stdin]

def expandRows(m):
    l = []
    for i in range(len(m)):
        if all([e == '.' for e in m[i]]):
            l.append(i)
    t = 0
    for i in l:
        m = m[:i+t] + [m[i+t]] + m[i+t:]
        t += 1
    return m

def expandColumns(m):
    l = []
    for i in range(len(m[0])):
        if all([e == '.' for e in [m[_][i] for _ in range(len(m))]]):
            l.append(i)
    t = 0
    for i in l:
        for e in range(len(m)):
            m[e] = m[e][:i+t] + '.' + m[e][i+t:]
        t += 1
    return m
        
r = expandColumns(r)
r = expandRows(r)
pos = []
for i in range(len(r)):
    for e in range(len(r[i])):
        if r[i][e] == '#':
            pos.append((i, e))
# print('\n'.join(r))
pos.sort()
tot = 0
for i in pos:
    for e in pos:
        if i >= e:
            continue
        # print(i, e, abs(i[0] - e[0]) + abs(i[1] - e[1]))
        tot += abs(i[0] - e[0]) + abs(i[1] - e[1]) # t = | x_1 - x_2 | + | y_1 - y_2 |
print(tot)