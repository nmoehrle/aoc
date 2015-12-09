f :: String -> Integer -> Integer
f (')':xs) 0 = 1
f ('(':xs) n = f xs (n + 1) + 1
f (')':xs) n = f xs (n - 1) + 1
f _ n = error "Invalid input"

main = readFile "../input.txt" >>= \s -> print $ f s 0
