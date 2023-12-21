import sys, time
m = ['.' + i.strip() + '.' for i in sys.stdin] # padding + line + padding
m = [''.join('.' for _ in range(len(m[0])))] + m + [''.join('.' for _ in range(len(m[0])))] # another padding
# print(m)
pos = (-1, -1)
try:
    for i in range(len(m)):
        for e in range(len(m[i])):
            if m[i].count('S') == 1:
                pos = (i, m[i].index('S'))
                raise IndexError # stop both fors
except:
    pass

def isNear(pos1, pos2):
    if pos1[0] == pos2[0] and abs(pos1[1] - pos2[1]) == 1:
        return True
    if pos1[1] == pos2[1] and abs(pos1[0] - pos2[0]) == 1:
        return True
    return False

p = [['.' for i in range(len(m[0]))] for _ in range(len(m))]
up, ups = 'S|JL', 'S7F|'
down, downs = 'S|7F', 'S|JL'
right, rights = 'S-LF', 'S-J7'
left, lefts = 'S-J7', 'S-LF'
visited = []
while pos not in visited:
    visited.append(pos)
    x, y = pos
    p[x][y] = m[x][y]
    
    if m[x-1][y] in ups and m[x][y] in up and (x-1, y) not in visited: # up
        pos = (x-1, y)
    elif m[x+1][y] in downs and m[x][y] in down and (x+1, y) not in visited: # down 
        pos = (x+1, y)
    elif m[x][y+1] in rights and m[x][y] in right and (x, y+1) not in visited: # right
        pos = (x, y+1)
    elif m[x][y-1] in lefts and m[x][y] in left and (x, y-1) not in visited: # left
        pos = (x, y-1)
         
print("\n".join(["".join(i) for i in p])) # TODO: finish lol