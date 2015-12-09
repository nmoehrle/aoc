with open('../input.txt') as f:
    floor = 0;
    for c in f.read():
        if c == '(':
            floor = floor + 1
        elif c == ')':
            floor = floor - 1
        else:
            raise Exception('Invalid input')
    print(floor)
