i = input()
m = {}

x, y = len(i)//2, len(i)//2
t = 0
for e in i:
    t += 1 if not m.get(f"{x};{y}") else 0
    m[f"{x};{y}"] = True
    if e == '>':
        x += 1
    elif e == 'v':
        y += 1
    elif e == '<':
        x -= 1
    elif e == '^':
        y -= 1
print(t)