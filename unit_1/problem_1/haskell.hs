divBy3or5 :: Int -> Bool
divBy3or5 x = x `mod` 3 == 0 || x `mod` 5 == 0

solution :: Int -> Int
solution n = sum $ filter divBy3or5 [1 .. n - 1]

result :: Int
result = solution 1000

main :: IO ()
main = print result
