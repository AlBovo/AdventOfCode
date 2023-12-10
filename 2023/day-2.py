import sys, re
c = sum = 0
for i in sys.stdin:
    c += 1
    blue = re.findall(r"\d+ blue", i)
    red = re.findall(r"\d+ red", i)
    green = re.findall(r"\d+ green", i)
    f = True
    
    for j in blue:
        t = int(j.replace(" blue", ""))
        if t > 14:
            f = False
            break
    
    for j in green:
        t = int(j.replace(" green", ""))
        if t > 13:
            f = False
            break
    
    for j in red:
        t = int(j.replace(" red", ""))
        if t > 12:
            f = False
            break
    
    if f:
        sum += c
print(sum)