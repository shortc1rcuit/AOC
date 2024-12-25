import Control.Monad
import Data.List
import System.IO

run_sol :: Show a => String -> (String -> a) -> IO ()
run_sol filename sol = do
    contents <- readFile filename
    print $ sol contents

{- part_1 input = sum (zipWith abs_diff (sort left) (sort right))
    where
        (left, right) = parse input
        abs_diff x y = abs (x - y) -}

--part_1 input = sum (zipWith ((abs .) . (-)) (sort left) (sort right))
--part_1 input = ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort) left right
--part_1 input = uncurry ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort) (parse input)
part_1 :: String -> Int
part_1 = (uncurry ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort)) . parse

{- part_2 :: String -> Int
part_2 input = sum (map (\x -> x * (count_eq right x)) left)
    where
        count_eq l x = length (filter (\y -> x == y) l)
        (left, right) = parse input -}

part_2 :: String -> Int
--part_2 input = sum (map ((*) <*> (count_eq right)) left)
part_2 = uncurry ((sum .) . flip (map . ((*) <*>) . count_eq)) . parse
    where
        count_eq = (length .) . flip (filter . (==))

{- parse :: String -> ([Int], [Int])
parse input = (left, right)
    where
        [left, right] = lists
        lists = transpose (map (\x -> map read (words x)) (lines input)) -}

parse :: String -> ([Int], [Int])
--parse input = (,) ((lists input) !! 0) ((lists input) !! 1)
--parse input = ((,) . (!! 0) . lists $ input) ((!! 1) . lists $ input)
parse = ((,) . (!! 0) . lists) <*> ((!! 1) . lists)
    where
        lists = transpose . map (map read . words) . lines