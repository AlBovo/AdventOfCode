import sys
EXPAND = 1000000-1
m = [i.strip() for i in sys.stdin]

def expandRows(t):
    l = []
    for i in range(len(m)):
        if all([e == '.' for e in m[i]]):
            l.append(i)
    for e in range(len(t)):
        r = 0
        for i in l:
            if t[e][0] > i:
                r += EXPAND
        t[e] = (t[e][0] + r, t[e][1])
    return t

def expandColumns(t):
    l = []
    for i in range(len(m[0])):
        if all([e == '.' for e in [m[_][i] for _ in range(len(m))]]):
            l.append(i)
    for e in range(len(t)):
        r = 0
        for i in l:
            if t[e][1] > i:
                r += EXPAND
        t[e] = (t[e][0], t[e][1] + r)
    return t

t = []
for i in range(len(m)):
    for e in range(len(m[0])):
        if m[i][e] == '#':
            t.append((i, e))
t.sort()
# print(t)
t = expandColumns(t)
t.sort()
t = expandRows(t)
t.sort()

# print(t)
tot = 0
for i in t:
    for e in t:
        if i >= e:
            continue
        tot += abs(i[0] - e[0]) + abs(i[1] - e[1])
print(tot)