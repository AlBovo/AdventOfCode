i = input()
p = 0
for r, j in enumerate(i):
    if j == '(':
        p += 1
    else:
        if p == 0:
            print(r+1)
            break
        else:
            p -= 1