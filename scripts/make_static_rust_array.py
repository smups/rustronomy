import numpy as np

# Simple script that creates a static rust array because there is no easy way to do this in rust

code = f"static {input('array name:')}: [{input('array primitive type:')}; &] = [\n"
start = int(input("array start (inclusive):"))
stop = int(input("array stop (exclusive):"))

code = code.replace("&", f"{stop - start}")

pos = 0
for val in np.arange(start, stop):
    if pos == 0: code += "    "
    code += f"{val}, "
    if pos == 9: 
        code += "\n"
        pos = -1
    pos += 1

code += "\n ];"

print(code)

with open("out.txt", "w") as file:
    file.write(code)