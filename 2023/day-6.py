import sys
m = [i for i in sys.stdin]
# r[0] = times
# r[1] = records

for i in range(len(m)):
    m[i] = m[i][m[i].index(':')+1:].strip().split()
    m[i] = [int(e) for e in m[i]]

c = 1
for i in range(len(m[0])):
    if c == 0:
        break
    t = 0
    for e in range(m[0][i]+1):
        if e * (m[0][i] - e) > m[1][i]:
            t += 1
    c *= t
print(c)