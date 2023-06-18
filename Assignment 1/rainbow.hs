{- 
CMPT 383 Assignment 1
Jiadi Luo
301354107
-}

import RainbowAssign
import qualified Data.Map as Map
import Data.Maybe


-- Parameters for the rainbow table.
pwLength, nLetters, width, height :: Int
filename :: FilePath
pwLength = 8            -- length of each password
nLetters = 5            -- number of letters to use in passwords: 5 -> a-e
width = 40             -- length of each chain in the table
height = 1000           -- number of "rows" in the table
filename = "table.txt"  -- filename to store the table


-- pwReduce: maps a hash value to an arbitrary password.
pwReduce :: Hash -> Passwd
pwReduce hash = map toLetter $ reverse $ pwReduceHelper $ fromEnum hash
    where
        pwReduceHelper :: Int -> [Int]
        pwReduceHelper hash' = take pwLength $ mod hash' nLetters : pwReduceHelper (div hash' nLetters)


-- rainbowTable: generates a rainbow table, given a list of initial passwords.
rainbowTable :: Int -> [Passwd] -> Map.Map Hash Passwd
rainbowTable widthLength passwords = Map.fromList [(x, y) | (x, y) <- rainbowTableWrapper]
    where
        rainbowTableWrapper :: [(Hash, Passwd)]
        rainbowTableWrapper = zip (rainbowTableHelper widthLength passwords) passwords

        rainbowTableHelper :: Int -> [Passwd] -> [Hash]
        rainbowTableHelper widthLength' passwords'
            | widthLength' == 0 = map pwHash passwords'
            | otherwise = rainbowTableHelper (widthLength'-1) (map pwReduce (map pwHash passwords'))


-- findPassword: reverses a hash to the corresponding password, if possible.
findPassword :: Map.Map Hash Passwd -> Int -> Hash -> Maybe Passwd
findPassword table widthLength hash = do
    correctPassword <- findCorrectPassword (findKeyInTable table widthLength hash) widthLength hash
    return correctPassword

findKeyInTable :: Map.Map Hash Passwd -> Int -> Hash -> [Maybe Passwd]
findKeyInTable table widthLength hash
    | widthLength < 0 = []
    | otherwise = Map.lookup hash table : findKeyInTable table (widthLength - 1) (pwHash (pwReduce hash))

findCorrectPassword :: [Maybe Passwd] -> Int -> Hash -> Maybe Passwd
findCorrectPassword [] _ _ = Nothing
findCorrectPassword (Nothing : xs) widthLength hash =
    findCorrectPassword xs widthLength hash
findCorrectPassword (Just keyInTable : xs) widthLength hash =
    case findCorrectPasswordHelper keyInTable widthLength hash of
        Just correctPassword -> Just correctPassword
        Nothing -> findCorrectPassword xs widthLength hash
    where
        findCorrectPasswordHelper :: Passwd -> Int -> Hash -> Maybe Passwd
        findCorrectPasswordHelper keyInTable' widthLength' hash'
            | widthLength' < 0 = Nothing
            | pwHash keyInTable' == hash' = Just keyInTable'
            | otherwise = findCorrectPasswordHelper (pwReduce (pwHash keyInTable')) (widthLength' - 1) hash'


-- Testing
generateTable :: IO ()
generateTable = do
    table <- buildTable rainbowTable nLetters pwLength width height
    writeTable table filename

test1 :: IO (Maybe Passwd)
test1 = do
    table <- readTable filename
    return (Map.lookup 0 table)

test2 :: Int -> IO ([Passwd], Int)
test2 n = do
    table <- readTable filename
    pws <- randomPasswords nLetters pwLength n
    let hs = map pwHash pws
    let result = Data.Maybe.mapMaybe (findPassword table width) hs
    return (result, length result)

main :: IO ()
main = do
    generateTable
    res1 <- test1
    print res1
    res2 <- test2 10000
    print res2
