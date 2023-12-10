import sys, re
m = [i for i in sys.stdin]

for i in range(len(m)):
    m[i] = int("".join(re.findall(r'\d+', m[i])))
t = 0
for i in range(m[0]//2 +1):
    if i * (m[0] - i) > m[1]:
        t += 1
print(t*2)