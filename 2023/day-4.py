import sys, re
m = [i for i in sys.stdin]
c = 0
for i in m:
    s = 0
    i = i.split('|')
    i[0] = i[0][i[0].index(':')+2:].strip()
    i[1] = i[1].strip()
    winning = [int(e) for e in re.findall(r'\d+', i[0])]
    my_numbs = [int(e) for e in re.findall(r'\d+', i[1])]
    
    for e in my_numbs:
        if e in winning:
            if s == 0:
                s = 1
            else:
                s *= 2
    c += s
print(c)