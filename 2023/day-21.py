import sys as sus
LENGTH = 64
m = []
x, y = 0, 0
for i, e in enumerate(sus.stdin):
    m.append([f for f in e.strip()])
    if e.count('S') == 1:
        assert x == 0 and y == 0 # just one starting point
        x, y = i, e.index('S')

queue = [(x, y, LENGTH+1)]
t = 0
while len(queue) > 0:
    x, y, l = queue[0]
    queue = queue[1:]
    
    if l == 0 or x < 0 or y < 0 or x >= len(m) or y >= len(m[0]):
        continue
    if m[x][y] in 'O#':
        continue
    
    if l % 2 != 0:
        t += 1
    m[x][y] = 'O'
    
    queue.append((x+1, y, l-1))
    queue.append((x-1, y, l-1))
    queue.append((x, y+1, l-1))
    queue.append((x, y-1, l-1))

print(t)
print("\n".join("".join(e) for e in m))