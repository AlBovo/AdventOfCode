import sys
m = [i.strip() for i in sys.stdin]
instr = m[0]
m = m[2:]
f = {}

for i in range(len(m)):
    m[i] = m[i].split(' = ')
    m[i][1] = m[i][1].replace('(', '').replace(')', '').split(', ')
    f[m[i][0]] = m[i][1]
del m
# print(f)

tot = 0
pos = 'AAA'
while True:
    for i in instr:
        if i == 'R':
            pos = f[pos][1]
        else:
            pos = f[pos][0]
        tot += 1
        
        if pos == 'ZZZ':
            print(tot)
            exit(0)