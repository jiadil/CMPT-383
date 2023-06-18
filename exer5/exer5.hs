{- 
CMPT 383 Exercise 5
Jiadi Luo
301354107
-}

import Data.Ratio

-- Built-in functions
myIterate :: (a -> a) -> a -> [a]
myIterate f x = x : myIterate f (f x)

mySplitAt :: Int -> [a] -> ([a], [a])
mySplitAt 0 lst = ([], lst) -- base case
mySplitAt _ [] = ([], []) -- empty list
mySplitAt n (x:xs) = (x : head, tail)
    where (head, tail) = mySplitAt (n - 1) xs


-- Rational numbers
rationalSum :: Int -> [Ratio Int]
rationalSum x = [numerator % (x - numerator) | numerator <- [1..x-1]]


-- Lowest Terms Only
rationalSumLowest :: Int -> [Ratio Int]
rationalSumLowest x = [numerator % (x - numerator) | numerator <- [1..x-1], gcd numerator (x - numerator) == 1]


-- All Rational Numbers
rationals :: [Ratio Int]
rationals = concatMap rationalSumLowest [1..]


-- Input/Output
-- split a list around a given separator value
splitAtSeparator :: Eq a => a -> [a] -> [[a]]
splitAtSeparator sep [] = []
splitAtSeparator sep content = first : splitAtSeparator sep rest
    where
    first = takeWhile (/= sep) content
    firstlen = length first
    rest = drop (firstlen+1) content

-- convert an integer-like string to an integer
readInt :: String -> Int
readInt = read

-- sumFile
sumFile :: IO ()
sumFile = do
    inputStr <- readFile "input.txt"
    let inputInt = map readInt (splitAtSeparator '\n' inputStr)
    let output = sum inputInt
    print output
