import sys
m = [i for i in sys.stdin]

def isGear(x: int, y: int) -> int:
    t1 = t2 = 1
    usedCoords = []
    for i in range(x-1, x+2, 1):
        for e in range(y-1, y+2, 1):
            if i < 0 or e < 0 or i >= len(m) or e >= len(m[0]) or [i, e] in usedCoords:
                continue
            elif m[i][e].isdecimal():
                s = ""
                for j in range(e, -1, -1):
                    if m[i][j].isdecimal():
                        usedCoords.append([i, j])
                        s = m[i][j] + s
                    else:
                        break
                
                for j in range(e+1, len(m[i])):
                    if m[i][j].isdecimal():
                        usedCoords.append([i, j])
                        s += m[i][j] 
                    else:
                        break
                # print(s)
                if t1 == 1:
                    t1 = int(s)
                elif t2 == 1:
                    t2 = int(s)
                else:
                    return 0

    return (t1 * t2 if t1 != 1 and t2 != 1 else 0)

c = 0
for i in range(len(m)):
    for e in range(len(m[i])):
        if m[i][e] == '*':
            c += isGear(i, e)
print(c)