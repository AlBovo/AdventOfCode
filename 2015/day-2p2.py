import sys
t = 0
for i in sys.stdin:
    i = i.split('x')
    l, w, h = int(i[0]), int(i[1]), int(i[2])
    s = l * w * h
    r = [l, w, h]
    r.sort()
    t += s + 2 * r[0] + 2 * r[1]
print(t)