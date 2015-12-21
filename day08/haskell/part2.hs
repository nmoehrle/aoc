f :: String -> Integer
f [] = 2
f ('"': xs) = 1 + f xs
f ('\\': xs) = 1 + f xs
f (x : xs) = f xs

main = readFile "../input.txt" >>= print . sum . (map f) . lines
