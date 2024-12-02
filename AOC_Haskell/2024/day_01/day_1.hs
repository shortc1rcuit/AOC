import Control.Monad
import Data.List
import System.IO

run_sol :: Show a => String -> (String -> a) -> IO ()
run_sol filename sol = do
    contents <- readFile filename
    print $ sol contents

{-
part_1 input = sum (zipWith abs_diff (sort left) (sort right))
    where
        (left, right) = parse input
        abs_diff x y = abs (x - y)
-}

--part_1 input = sum (zipWith ((abs .) . (-)) (sort left) (sort right))
--part_1 input = ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort) left right
--part_1 input = uncurry ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort) (parse input)
part_1 :: String -> Int
part_1 = (uncurry ((sum .) . (. sort) . zipWith ((abs .) . (-)) . sort)) . parse

{- parse :: String -> ([Int], [Int])
parse input = (left, right)
    where
        [left, right] = lists
        lists = transpose (map (\x -> map read (words x)) (lines input)) -}

--parse :: String -> ([Int], [Int])
--parse input = (,) ((lists input) !! 0) ((lists input) !! 1)
--parse input = ((,) . (!! 0) . lists $ input) ((!! 1) . lists $ input)
parse = ap ((,) . (!! 0) . lists) ((!! 1) . lists)
    where
        lists = transpose . map (map read . words) . lines