from z3 import *

with open('inputpy.txt', 'r') as f:
    output = []
    for line in f.readlines():
        output.append([int(s) for s in line.strip().split(",")])
    variables = []
    variables.append(Int('ax'))
    ax = variables[-1]
    variables.append(Int('ay'))
    ay = variables[-1]
    variables.append(Int('az'))
    az = variables[-1]
    variables.append(Int('bx'))
    bx = variables[-1]
    variables.append(Int('by'))
    by = variables[-1]
    variables.append(Int('bz'))
    bz = variables[-1]
    solver = Solver()
    for (i, arr) in enumerate(output):
        variables.append(Int(f'cx{i}'));
        cx = variables[-1]
        variables.append(Int(f'cy{i}'));
        cy = variables[-1]
        variables.append(Int(f'cz{i}'));
        cz = variables[-1]
        variables.append(Int(f'dx{i}'));
        dx = variables[-1]
        variables.append(Int(f'dy{i}'));
        dy = variables[-1]
        variables.append(Int(f'dz{i}'));
        dz = variables[-1]
        variables.append(Int(f't{i}'));
        t = variables[-1]
        solver.add(And(cx == arr[0]))
        solver.add(And(cy == arr[1]))
        solver.add(And(cz == arr[2]))
        solver.add(And(dx == arr[3]))
        solver.add(And(dy == arr[4]))
        solver.add(And(dz == arr[5]))
        solver.add(And(t >= 0))
        solver.add(And(ax - cx == (dx - bx) * t))
        solver.add(And(ay - cy == (dy - by) * t))
        solver.add(And(az - cz == (dz - bz) * t))
    solver.check()
    res = solver.model()
    print(res[ax].as_long() + res[ay].as_long() + res[az].as_long())
