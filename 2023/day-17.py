from queue import PriorityQueue
import sys
t = [[int(e) for e in i.strip()] for i in sys.stdin]
r = [[{r : int(1e9) for r in range(4)} for _ in i] for i in t]

# class Status:
#     x, y, z = 0, 0, 0
#     direction = -1 # 0:top 1:right 2:bottom 3:left
#     def __init__(self, x, y, z, direction) -> None:
#         self.x = x
#         self.y = y
#         self.z = z
#         self.direction = direction
q = PriorityQueue()
q.put((0, (0, 0, 0, 2)))
q.put((0, (0, 0, 0, 1)))
r[0][0][0] = 0

while not q.empty():
    p = q.get()
    
    x, y, z, direction = p[1]
    if z >= 2:
        if direction in [0, 2]:
            if y > 0 and r[x][y-1][0] > r[x][y][z] + t[x][y]:
                r[x][y-1][0] = r[x][y][z] + t[x][y]
                q.put((r[x][y-1][0], (x, y-1, 0, 3)))
            if y < len(t[0]) - 1 and r[x][y+1][0] > r[x][y][z] + t[x][y]:
                r[x][y+1][0] = r[x][y][z] + t[x][y]
                q.put((r[x][y+1][0], (x, y+1, 0, 1)))
        else:
            if x > 0 and r[x-1][y][0] > r[x][y][z] + t[x][y]:
                r[x-1][y][0] = r[x][y][z] + t[x][y]
                q.put((r[x-1][y][0], (x-1, y, 0, 0)))
            if x < len(t) - 1 and r[x+1][y][0] > r[x][y][z] + t[x][y]:
                r[x+1][y][z] = r[x][y][0] + t[x][y]
                q.put((r[x+1][y][0], (x+1, y, 0, 2)))
    else:
        if y > 0 and r[x][y-1][0 if direction != 3 else z+1] > r[x][y][z] + t[x][y]:
            r[x][y-1][0 if direction != 3 else z+1] = r[x][y][z] + t[x][y]
            q.put((r[x][y-1][0 if direction != 3 else z+1], (x, y-1, 0 if direction != 3 else z+1, 3)))
        if y < len(t[0]) - 1 and r[x][y+1][0 if direction != 1 else z+1] > r[x][y][z] + t[x][y]:
            r[x][y+1][0 if direction != 1 else z+1] = r[x][y][z] + t[x][y]
            q.put((r[x][y+1][0 if direction != 1 else z+1], (x, y+1, 0 if direction != 1 else z+1, 1)))
        if x > 0 and r[x-1][y][0 if direction != 0 else z+1] > r[x][y][z] + t[x][y]:
            r[x-1][y][0 if direction != 0 else z+1] = r[x][y][z] + t[x][y]+t[0][0]
            q.put((r[x-1][y][0 if direction != 0 else z+1], (x-1, y, 0 if direction != 0 else z+1, 0)))
        if x <  len(t) - 1 and r[x+1][y][0 if direction != 2 else z+1] > r[x][y][z] + t[x][y]:
            r[x+1][y][0 if direction != 2 else z+1] = r[x][y][z] + t[x][y]
            q.put((r[x+1][y][0 if direction != 2 else z+1], (x+1, y, 0 if direction != 2 else z+1, 2)))
print(r[len(t)-1][len(t[0])-1]) # TODO: finish lol