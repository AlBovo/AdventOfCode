import sys
t = [[_ for _ in i.strip()] for i in sys.stdin]

for i in range(len(t)):
    for e in range(len(t[i])):
        if t[i][e] == 'O':
            y = i
            while y > 0:
                if t[y-1][e] == '.':
                    t[y-1][e] = 'O'
                    t[y][e] = '.'
                    y -= 1
                else:
                    break
# print("\n".join(["".join(i) for i in t]))
sum = 0
for i in range(len(t)):
    for e in t[i]:
        if e == 'O':
            sum += len(t) - i
print(sum)