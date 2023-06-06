half_of x = x/2
add_double x y = 2 * (x + y)

isBig a = a >= 100

listify x y = [x, y]

{- 
Pattern Matching - recursive 
-}
isZero 0 = True
isZero _ = False
-- use: isZero 0, isZero 1, isZero (1-1)

myAnd False _ = False
myAnd True a = a

listEqual [] [] = True
listEqual (x:xs) (y:ys) = x == y && listEqual xs ys
listEqual [] (_:_) = False
listEqual (_:_) [] = False
-- Another way to write the above two lines: listEqual _ _ = False
-- use: listEqual [1,2,3] [1,2,3]

hasEvenLength [] = True
hasEvenLength [_] = False
hasEvenLength (_:_:xs) = hasEvenLength xs
-- use: hasEvenLength [1,2,3,4,5]


{-
Conditionals
-}
mySignum x
    | x < 0 = -1
    | x > 0 = 1
    | otherwise = 0
-- use: mySignum 0, mySignum 1, mySignum (-1)

word n = case n of 
    1 -> "one"
    2 -> "two"
    3 -> "three"
    _ -> "unknown"
-- use: word 1, word 2, word 3, word 4

wordWithX n = (case n of 
    1 -> "one"
    2 -> "two"
    3 -> "three"
    _ -> "unknown") ++ "X"
wordWithX2 n = (word n) ++ "X"
-- use: wordWithX 1, wordWithX 2, wordWithX 3, wordWithX 4

describeList xs = "The list is " ++ case xs of
    _:_:_:_:_ -> "a longer list." -- >= 4 elements
    _:_ -> "a short list." -- >= 1 element
    [] -> "empty."
-- use: describeList [], describeList [1], describeList [1,2], describeList [1,2,3,4,5]


{-
Repetition - recursive
-}
myLength [] = 0
myLength (_:xs) = 1 + myLength xs


{- 
List Comprehension
-}
-- [ x^2+1 | x <- [1..10], even x, x>2 ]
-- [ succ c | c <- "abcde" ] succ is "successor", the next one

firstSquares = [ x^2 | x <- [1..10] ]
-- use: firstSquares

firstSquaresInput n = [ x^2 | x <- [1..n] ]
-- use: firstSquaresInput 10




{-
Let and Where
-}
-- Let
-- let x=4 in x*2
-- Quick Sort--
qs [] = []
qs (x:xs) = smaller ++ [x] ++ larger
    where smaller = qs [a | a <- xs, a <= x]
          larger = qs [a | a <- xs, a > x]
-- use: qs [2,4,1,5,3,10,5]

-- Where
qs' [] = []
qs' (x:xs) = 
    let smaller = qs' [a | a <- xs, a <= x]
        larger = qs' [a | a <- xs, a > x]
    in smaller ++ [x] ++ larger


{-
Recursion
-}
myPower _ 0 = 1
myPower x y = x * myPower x (y-1)

myPower' x y
  | y==0      = 1
  | even y    = half*half
  | odd y     = x*half*half
  where half = myPower' x (div y 2)


{-
List processing
-}
myReverse xs = foldl prefix [] xs
    where prefix xs x = x:xs
myReverse' xs = foldr postfix [] xs
    where postfix x xs = xs ++ [x]

fun :: Int -> Int
fun x = div x 2