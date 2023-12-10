import sys
tot = 0
for i in sys.stdin:
    t = 0
    for j in str(i):
        if j.isdigit():
            t += int(j)*10
            break
    for j in str(i)[::-1]:
        if j.isdigit():
            t += int(j)
            break
    tot += t
print(tot)