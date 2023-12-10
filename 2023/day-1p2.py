import sys
tot = 0
for i in sys.stdin:
    i = str(i)
    i = i.replace("one", "o1e") # shitty replaces
    i = i.replace("two", "t2o")
    i = i.replace("three", "t3e")
    i = i.replace("four", "4")
    i = i.replace("five", "5e")
    i = i.replace("six", "6")
    i = i.replace("seven", "7n")
    i = i.replace("eight", "e8t")
    i = i.replace("nine", "9e")
    
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