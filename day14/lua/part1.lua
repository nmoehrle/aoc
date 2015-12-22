function distance (speed, stamina, rest, time)
    local cycle = stamina + rest;
    local cycles = (time // cycle);
    return cycles * stamina * speed
        + math.min(stamina, time - cycles * cycle) * speed;
end

local max = 0;
for line in io.lines('../input.txt') do
    speed, stamina, rest = string.match(line, '(%d+) .* (%d+) .* (%d+)');
    max = math.max(max,
        distance(tonumber(speed), tonumber(stamina), tonumber(rest), 2503));
end
print(max);
