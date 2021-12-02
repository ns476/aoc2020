with open("input") as f:
    lines = list(map(lambda x: x.split(), f.readlines()))

aim = 0
horiz = 0
depth = 0
for (command, amount) in lines:
    if command == 'forward':
        horiz += int(amount)
        depth += int(amount) * aim
    elif command == 'down':
        aim += int(amount)
    elif command == 'up':
        aim -= int(amount)

print(depth * horiz)
