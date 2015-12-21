e :: String -> Integer
e ('x': d1: d2: xs) = 2 + f xs
e (x : xs) = f xs
e [] = error "Invalid input"

f :: String -> Integer
f [] = 0
f ('"': xs) = 1 + f xs
f ('\\': xs) = 1 + e xs
f (x : xs) = f xs

main = readFile "../input.txt" >>= print . sum . (map f) . lines
