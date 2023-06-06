{- 
CMPT 383 Exercise 1
Jiadi Luo
301354107
-}

-- Primers and Divisors
divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n `div` 2)], n `mod` i == 0]
primes :: Int -> [Int]
primes n = [i | i <- [2..n], divisors i == []]

-- Pythagorean Triples (list comprehension)
pythagorean :: Int -> [(Int, Int, Int)]
pythagorean n = [(a,b,c) | c <- [1..n], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2]

-- Joining Strings (pattern matching on the arguments)
join :: String -> [String] -> String
join _ [] = ""
join _ [x] = x
join separator (x:xs) = x ++ separator ++ (join separator xs)

-- Factorial with a fold (not resursive)
fact' :: Int -> Int
fact' x = foldl (*) 1 [1..x]

-- Tail Recursive Hailstone
-- Hailstone Function from last week (guarded expression)
hailstone x
    | even x = div x 2
    | otherwise = 3 * x + 1

hailLen n = hailTail 0 n
  where
    hailTail a 1 = a
    hailTail a n = hailTail (a+1) (hailstone n)
