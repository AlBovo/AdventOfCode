import sys as sus

maxXPos, maxYPos = 0, 0
maxXNeg, maxYNeg = 0, 0
x = y = t = 0
d = []
for i in sus.stdin:
    i = i.strip().split(' ')
    # print(i)
    if i[0] == 'R':
        y += int(i[1])
    elif i[0] == 'L':
        y -= int(i[1])
    elif i[0] == 'U':
        x -= int(i[1])
    else: # down
        x += int(i[1])
    t += int(i[1])
    d.append((i[0], int(i[1])))
    maxXPos = max(x, maxXPos)
    maxYPos = max(y, maxYPos)
    maxXNeg = min(x, maxXNeg)
    maxYNeg = min(y, maxYNeg)
    
# print(maxXPos, maxXNeg, maxYPos, maxYNeg)
maxX, maxY = maxXPos + abs(maxXNeg), maxYPos + abs(maxYNeg)
m = [[False for _ in range(maxY * 2 + 1)] for i in range(maxX * 2 + 1)]
x, y = maxX, maxY
for i in d:
    # print(x, y, i)
    if i[0] == 'R':
        for e in range(0, i[1]+1):
            m[x][y + e] = True
        y += i[1]
    elif i[0] == 'L':
        for e in range(0, i[1]+1):
            m[x][y - e] = True
        y -= i[1]
    elif i[0] == 'U':
        for e in range(0, i[1]+1):
            m[x - e][y] = True
        x -= i[1]
    else: # down
        for e in range(0, i[1]+1):
            m[x + e][y] = True
        x += i[1]
    assert x >= 0 and y >= 0
    # print("\n".join("".join('#' if i else '.' for i in e) for e in m))

def dfs(x: int, y: int):
    if x < 0 or y < 0 or x >= len(m) or y >= len(m[0]) or m[x][y]:
        return 0
    m[x][y] = True
    tot = 1
    for i in range(-1, 2):
        for e in range(-1, 2):
            tot += dfs(x+i, y+e)
    return tot
sus.setrecursionlimit(1000000)
dfs(0, 0) # just a bit slow
# print("\n".join("".join('#' if i else '.' for i in e) for e in m))
for i in range(len(m)):
    if m[i].count(False) > 0:
        print(dfs(i, m[i].index(False)) + t)
        exit(0)