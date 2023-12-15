import sys
t = [[_ for _ in i.strip()] for i in sys.stdin]

pos = []
for i in range(len(t)):
    for e in range(len(t[i])):
        if t[i][e] == 'O':
            pos.append((i, e))

c = 0
def cycle(pos: list) -> list:
    # print("\n".join(["".join(i) for i in t]), end="\n\n")
    pos.sort(key=lambda x: x[0])
    
    for _ in range(len(pos)):
        y, x = pos[_]
        # roll north
        while y > 0:
            if t[y-1][x] == '.':
                t[y-1][x] = 'O'
                t[y][x] = '.'
                y -= 1
            else:
                break
        pos[_] = (y, x)
        
    # print("\n".join(["".join(i) for i in t]), end="\n\n")
    pos.sort(key=lambda x: x[1])

    for _ in range(len(pos)):
        y, x = pos[_]
        # roll west
        while x > 0:
            if t[y][x-1] == '.':
                t[y][x-1] = 'O'
                t[y][x] = '.'
                x -= 1
            else:
                break
        pos[_] = (y, x)

    # print("\n".join(["".join(i) for i in t]), end="\n\n")
    pos.sort(key=lambda x: x[0], reverse=True)
    
    for _ in range(len(pos)):
        y, x = pos[_]
        # roll south
        while y < len(t)-1:
            if t[y+1][x] == '.':
                t[y+1][x] = 'O'
                t[y][x] = '.'
                y += 1
            else:
                break
        pos[_] = (y, x)
        
    # print("\n".join(["".join(i) for i in t]), end="\n\n")
    pos.sort(key=lambda x: x[1], reverse=True)

    for _ in range(len(pos)):
        y, x = pos[_]
        # roll east
        while x < len(t[0])-1:
            if t[y][x+1] == '.':
                t[y][x+1] = 'O'
                t[y][x] = '.'
                x += 1
            else:
                break
        pos[_] = (y, x)
    
    # print("\n".join(["".join(i) for i in t]), end="\n\n")
    
    return pos
    
for i in range(1000000000//1000000): # just a bit big (strange bound)
    pos = cycle(pos)

    try:
        while True:
            pos.remove((-1, -1))
    except:
        pass
    
    # print("\n".join(["".join(e) for e in t]), end="\n\n")
    if len(pos) == 0:
        break

sum = 0
for i in range(len(t)):
    for e in t[i]:
        if e == 'O':
            sum += len(t) - i
print(sum)