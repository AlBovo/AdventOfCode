import sys as sus
m = [[_ for _ in i.strip()] for i in sus.stdin]
vis = [[False for i in e] for e in m]

def dfs(x, y):
    if (x, y) == (len(m) - 1, len(m[0]) - 2) or m[x][y] == '#' or vis[x][y]: # forest or wall
        return 0
    
    vis[x][y] = True
    
    if m[x][y] == '>':
        t = dfs(x, y+1) + 1
    elif m[x][y] == '<':
        t = dfs(x, y-1) + 1
    elif m[x][y] == 'v':
        t = dfs(x+1, y) + 1
    elif (x, y) == (0, 1):
        t = dfs(x+1, y) + 1
    else:
        t = max([dfs(x+1, y), dfs(x, y+1), dfs(x-1, y), dfs(x, y-1)]) + 1
    vis[x][y] = False
    return t

sus.setrecursionlimit(100000)
print(dfs(0, 1)) # starting point