import sys, math
m = [i.strip() for i in sys.stdin]
instr = m[0]
starts = []
m = m[2:]
f = {}

for i in range(len(m)):
    m[i] = m[i].split(' = ')
    m[i][1] = m[i][1].replace('(', '').replace(')', '').split(', ')
    f[m[i][0]] = m[i][1]
    if m[i][0].endswith('A'):
        starts.append(m[i][0])
del m
tot = 1
for i in starts:
    cont = True
    t = 0
    pos = i
    while cont:
        for e in instr:
            if e == 'R':
                pos = f[pos][1]
            else:
                pos = f[pos][0]
            t += 1

            if pos.endswith('Z'):
                cont = False
                break
    # print(i, t)
    tot = math.lcm(tot, t)
print(tot)