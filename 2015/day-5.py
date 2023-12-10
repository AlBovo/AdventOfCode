import sys, re
f = ["ab", "cd", "pq", "xy"]
c = 0
for i in sys.stdin:
    if len(re.findall(r"[aeiou]", i)) < 3:
        continue
    if len(re.findall(r"(\w)\1", i)) == 0:
        continue
    
    r = False    
    for e in f:
        if e in i:
            r = True
            break
    if not r:
        c += 1
print(c)