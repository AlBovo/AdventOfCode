import sys as sus

maxXPos, maxYPos = 0, 0
maxXNeg, maxYNeg = 0, 0
x, y = 0, 0
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
    d.append((i[0], int(i[1])))
    maxXPos = max(x, maxXPos)
    maxYPos = max(y, maxYPos)
    maxXNeg = min(x, maxXPos)
    maxYNeg = min(y, maxYPos)

maxX, maxY = maxXPos + maxXNeg, maxYPos + maxYNeg
m = [[False for _ in range(maxY * 2 + 2)] for i in range(maxX * 2 + 2)]
x, y = maxX + 1, maxY + 1
for i in d:
    print(x, y, i)
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

# print("\n".join("".join('#' if i else '.' for i in e) for e in m))