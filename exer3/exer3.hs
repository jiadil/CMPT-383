{- 
CMPT 383 Exercise 3
Jiadi Luo
301354107
-}

import Data.Time.Calendar
import Data.Time.Calendar.OrdinalDate


-- Merging
merge :: Ord t => [t] -> [t] -> [t]
merge x [] = x
merge [] y = y
merge (x:xs) (y:ys)
    | x <= y = x : merge xs (y:ys)
    | otherwise = y : merge (x:xs) ys


-- Merge Sort
mergeSort :: Ord t => [t] -> [t]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort firstHalf) (mergeSort secondHalf)
  where (firstHalf, secondHalf) = splitAt (div (length xs) 2) xs


-- Haskell Library and Dates
-- daysInYear
daysInYear :: Integer -> [Day]
daysInYear year = [jan1..dec31]
  where jan1 = fromGregorian year 1 1
        dec31 = fromGregorian year 12 31

-- isFriday
isFriday :: Day -> Bool
isFriday inputDate = week == 5
    where (_, week) = mondayStartWeek inputDate

-- Primers and Divisors (from exer2)
divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n `div` 2)], n `mod` i == 0]

isPrimeNumber :: Int -> Bool
isPrimeNumber x = divisors x == []

getDay :: (Integer, Int, Int) -> Int
getDay (y,m,d) = d

-- isPrimeDay
isPrimeDay :: Day -> Bool
isPrimeDay inputDate = isPrimeNumber day
  where day = getDay (toGregorian inputDate)
    
-- primeFridays
primeFridays :: Integer -> [Day]
primeFridays year = filter isPrimeDay (filter isFriday (daysInYear year))
