module Main where

import PrettyHTML.Main

main :: IO ()
main = putStrLn $ makeHtml "" "hello"