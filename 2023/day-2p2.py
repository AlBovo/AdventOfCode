import sys, re
c = sum = 0
for i in sys.stdin:
    c += 1
    blue = re.findall(r"\d+ blue", i)
    red = re.findall(r"\d+ red", i)
    green = re.findall(r"\d+ green", i)
    
    bMax = -1
    for j in blue:
        t = int(j.replace(" blue", ""))
        bMax = max(bMax, t)
    
    gMax = -1
    for j in green:
        t = int(j.replace(" green", ""))
        gMax = max(gMax, t)
    
    rMax = -1
    for j in red:
        t = int(j.replace(" red", ""))
        rMax = max(rMax, t)
    
    sum += bMax * gMax * rMax
print(sum)