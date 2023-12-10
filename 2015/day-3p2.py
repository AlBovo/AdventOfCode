i = input()
m = {}

x1, y1 = len(i)//2, len(i)//2
x2, y2 = len(i)//2, len(i)//2

for e, r in enumerate(i):
    if e % 2 == 0:
        m[f"{x1};{y1}"] = True
        if r == '^':
            y1 -= 1
        elif r == '<':
            x1 -= 1
        elif r == '>':
            x1 += 1
        else:
            y1 += 1
    else:
        m[f"{x2};{y2}"] = True
        if r == '^':
            y2 -= 1
        elif r == '<':
            x2 -= 1
        elif r == '>':
            x2 += 1
        else:
            y2 += 1
print(len(m))