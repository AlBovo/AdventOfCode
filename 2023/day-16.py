import sys
# 0:top, 1:right, 2:bottom, 3:left
mat = [[e for e in i.strip()] for i in sys.stdin]
vis = [[[False] * 4 for _ in range(len(mat[0]))] for i in range(len(mat))] # four directions
queue = [(0, 0, 1)]

while len(queue) > 0: # kind of bfs
    pos = queue[0]
    # print(pos)
    queue = queue[1:]
    # print(pos, queue)
    
    if pos[0] < 0 or pos[1] < 0 or pos[0] >= len(mat) or pos[1] >= len(mat[0]):
        continue
    
    if vis[pos[0]][pos[1]][pos[2]]:
        continue

    vis[pos[0]][pos[1]][pos[2]] = True
    ch = mat[pos[0]][pos[1]]
    if ch == '.' or (ch == '-' and pos[2] in [1, 3]) or (ch == '|' and pos[2] in [0, 2]):
        if pos[2] == 0: # top
            queue.append((pos[0]-1, pos[1], pos[2]))
        elif pos[2] == 1: # right
            queue.append((pos[0], pos[1]+1, pos[2]))
        elif pos[2] == 2: # bottom
            queue.append((pos[0]+1, pos[1], pos[2]))
        else: # left
            queue.append((pos[0], pos[1]-1, pos[2]))
    elif ch == '-':
        queue.append((pos[0], pos[1]-1, 3))
        queue.append((pos[0], pos[1]+1, 1))
    elif ch == '|':
        queue.append((pos[0]-1, pos[1], 0))
        queue.append((pos[0]+1, pos[1], 2))
    elif ch == '\\':
        if pos[2] == 0: # top
            queue.append((pos[0], pos[1]-1, 3))
        elif pos[2] == 1: # right
            queue.append((pos[0]+1, pos[1], 2))
        elif pos[2] == 2: # bottom
            queue.append((pos[0], pos[1]+1, 1))
        else: # left
            queue.append((pos[0]-1, pos[1], 0))
    else: # /
        if pos[2] == 0: # top
            queue.append((pos[0], pos[1]+1, 1))
        elif pos[2] == 1: # right
            queue.append((pos[0]-1, pos[1], 0))
        elif pos[2] == 2: # bottom
            queue.append((pos[0], pos[1]-1, 3))
        else: # left
            queue.append((pos[0]+1, pos[1], 2))

sum = 0
for i in range(len(vis)):
    for e in vis[i]:
        if any(i for i in e):
            sum += 1
            
print(sum)