module PrettyHTML.Main where

slice list start end = take (end - start + 1) (drop start list)

close tag = "</" <> slice tag 1 (length tag)
wrap tag content = tag <> content <> close tag

wrapHtml  = wrap "<html>"
wrapBody  = wrap "<body>"
wrapHead  = wrap "<head>"
wrapTitle = wrap "<title>"

makeHtml title body = wrapHtml $ (wrapHead $ wrapTitle title) <> wrapBody body