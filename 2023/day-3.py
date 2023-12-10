import sys, re
m = [i.strip() for i in sys.stdin]
notCorrect = '0123456789.'
numbers = []

for i, s in enumerate(m):
    n = re.finditer(r"\d+", s)
    for e in n:
        numbers.append([int(e.group()), int(e.start()), int(e.end()-1), i])

c = 0
for i in numbers:
    start, end, line = i[1], i[2], i[3]
    valid = False
    
    for x in range(line-1, line+2):
        for y in range(start-1, end+2):
            if x < 0 or y < 0 or x >= len(m) or y >= len(m[0]) or m[x][y] in notCorrect:
                continue
            valid = True
            break
        if valid:
            c += i[0]
            # print(i[0])
            break
print(c)