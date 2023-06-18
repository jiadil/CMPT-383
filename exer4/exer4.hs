{- 
CMPT 383 Exercise 4
Jiadi Luo
301354107
-}

-- Pascal's Triangle
-- zip helper function
zipValue :: [Int] -> [Int]
zipValue prev = map (\(x,y) -> x+y) (zip prev (tail prev))
-- pascal
pascal :: Int -> [Int]
pascal 0 = [1]
pascal 1 = [1,1]
pascal n = [1] ++ zipValue (pascal (n-1)) ++ [1]


-- Pointfree Addition
addPair :: Num a => (a, a) -> a
addPair = uncurry (+)


-- Pointfree Filtering
withoutZeros :: (Eq a, Num a) => [a] -> [a]
withoutZeros = filter (/= 0)


-- Searching? Maybe?
-- findElt helper function
findIndex :: Eq a => a -> [a] -> Maybe Int
findIndex _ [] = Nothing
findIndex x (y:ys)
    | x == y = Just 0
    | otherwise = do
        pos <- findIndex x ys
        return (pos + 1)
-- findElt
findElt :: Eq a => a -> [a] -> Maybe Int
findElt v1 xs = do
    pos1 <- findIndex v1 xs
    return (pos1)
