f :: String -> Integer
f [] = 0
f ('(':xs) = f xs + 1
f (')':xs) = f xs - 1
f _ = error "Invalid input"

main = readFile "../input.txt" >>= print . f
