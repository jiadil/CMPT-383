{- 
CMPT 383 Exercise 1
Jiadi Luo
301354107
-}


-- First Haskell Code
det a b c = b^2 - 4*a*c
quadsol1 a b c = (-b - sqrt (det a b c))/2*a
quadsol2 a b c = (-b + sqrt (det a b c))/2*a


-- Writing Your First Code
third_a x = head (drop 2 x) -- built-in list indexing operator
third_b (_:_:x:_) = x -- pattern matching


-- Factorial
fact x
    | x == 0 = 1
    | otherwise = x * fact (x-1)


-- Hailstone Function (guarded expression)
hailstone x
    | even x = div x 2
    | otherwise = 3 * x + 1


-- Hailstone Length (pattern matching)
hailLen 1 = 0
hailLen x = 1 + hailLen (hailstone x)
