f :: Char -> Integer -> String ->  String
f c n [] = (show n) ++ c : []
f c 0 (x : xs) = f x 1 xs
f c n (x : xs) = if x == c
    then f c (n + 1) xs
    else (show n) ++ c : f x 1 xs

main = print (length (iterate (f ' ' 0) "3113322113" !! 50))
