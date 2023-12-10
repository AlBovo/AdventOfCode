from hashlib import md5
s = "iwrupvqb"
i = 1
while True:
    hash = md5((s + str(i)).encode()).hexdigest()
    if hash.startswith("0"*6):
        print(i)
        break
    i += 1