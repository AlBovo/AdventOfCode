import sys
t = 0
for i in sys.stdin:
    i = i.split('x')
    l, w, h = int(i[0]), int(i[1]), int(i[2])
    s = 2*l*w + 2*w*h + 2*h*l
    r = [l, w, h]
    r.sort()
    t += s + r[0] * r[1]
print(t)