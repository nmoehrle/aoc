with open('../input.txt') as f:
    floor = 0;
    for n, c in enumerate(f.read()):
        if floor == -1:
            print(n)
            break
        if c == '(':
            floor = floor + 1
        elif c == ')':
            floor = floor - 1
        else:
            raise Exception('Invalid input')
