with open("input") as f:
    lines = list(map(lambda x: x.split(), f.readlines()))

depth = 0
horiz = 0
for (command, amount) in lines:
    if command == "forward":
        horiz += int(amount)
    elif command == "down":
        depth += int(amount)
    elif command == "up":
        depth -= int(amount)

print(depth * horiz)
