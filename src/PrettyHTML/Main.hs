module PrettyHTML.Main (
  wrapHTML
  ) where

wrapHTML content = "<html><body>" <> content <> "</body></html>"