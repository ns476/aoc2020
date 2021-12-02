from collections import Counter

with open("input") as f:
    lines = f.readlines()

print(Counter(map(lambda l: int(l[1]) > int(l[0]), zip(lines, lines[1:])))[True])
