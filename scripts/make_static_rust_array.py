# Copyright (C) 2021 Raúl Wolters

# This file is part of rustronomy.

# rustronomy is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# rustronomy is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with rustronomy.  If not, see <http://www.gnu.org/licenses/>.

import numpy as np

# Simple script that creates a static rust array because there is no easy way to do this in rust
# (which is dumb)

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