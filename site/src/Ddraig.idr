-- Ddraig.idr - Dependently typed static site generator in Idris 2
--
-- "Ddraig" (Welsh for Dragon) - Types that breathe fire

module Ddraig

import Data.List
import Data.String
import System
import System.File

-- ============================================================================
-- Frontmatter Types
-- ============================================================================

public export
record Frontmatter where
  constructor MkFrontmatter
  title : String
  date : String
  tags : List String
  draft : Bool
  template : String

export
emptyFrontmatter : Frontmatter
emptyFrontmatter = MkFrontmatter "" "" [] False "default"

-- ============================================================================
-- String Utilities (all self-contained)
-- ============================================================================

strHasPrefix : String -> String -> Bool
strHasPrefix pre str =
  let plen = length pre
      slen = length str
  in if plen > slen
       then False
       else substr 0 plen str == pre

strDropPrefix : String -> String -> String
strDropPrefix pre str =
  if strHasPrefix pre str
     then substr (length pre) (length str) str
     else str

strSplitOn : Char -> String -> List String
strSplitOn delim str = go (unpack str) [] []
  where
    go : List Char -> List Char -> List String -> List String
    go [] acc result = reverse (pack (reverse acc) :: result)
    go (x :: xs) acc result =
      if x == delim
         then go xs [] (pack (reverse acc) :: result)
         else go xs (x :: acc) result

strEscape : String -> String
strEscape s = concat (map esc (unpack s))
  where
    esc : Char -> String
    esc '<' = "&lt;"
    esc '>' = "&gt;"
    esc '&' = "&amp;"
    esc '"' = "&quot;"
    esc c = singleton c

-- ============================================================================
-- Frontmatter Parser
-- ============================================================================

parseFmLine : String -> Frontmatter -> Frontmatter
parseFmLine line fm =
  case break (== ':') (unpack line) of
    (keyChars, []) => fm
    (keyChars, _ :: valueChars) =>
      let key = trim (pack keyChars)
          value = trim (pack valueChars)
      in case key of
           "title" => { title := value } fm
           "date" => { date := value } fm
           "template" => { template := value } fm
           "draft" => { draft := (value == "true" || value == "yes") } fm
           "tags" =>
             let tagStr = if strHasPrefix "[" value
                            then substr 1 (length value `minus` 2) value
                            else value
                 tagList = map trim (strSplitOn ',' tagStr)
             in { tags := filter (\s => length s > 0) tagList } fm
           _ => fm

export
parseFrontmatter : String -> (Frontmatter, String)
parseFrontmatter content =
  let allLines = lines content
  in case allLines of
       [] => (emptyFrontmatter, content)
       (first :: rest) =>
         if trim first /= "---"
            then (emptyFrontmatter, content)
            else findEnd rest emptyFrontmatter
  where
    findEnd : List String -> Frontmatter -> (Frontmatter, String)
    findEnd [] fm = (fm, "")
    findEnd (l :: ls) fm =
      if trim l == "---"
         then (fm, unlines ls)
         else findEnd ls (parseFmLine l fm)

-- ============================================================================
-- Markdown Parser
-- ============================================================================

record ParserState where
  constructor MkState
  html : String
  inPara : Bool
  inCode : Bool
  inList : Bool
  orderedList : Bool

initState : ParserState
initState = MkState "" False False False False

closePara : ParserState -> ParserState
closePara st =
  if st.inPara
     then { html $= (++ "</p>\n"), inPara := False } st
     else st

closeUl : ParserState -> ParserState
closeUl st =
  if st.inList
     then let tag = if st.orderedList then "</ol>\n" else "</ul>\n"
          in { html $= (++ tag), inList := False } st
     else st

doInline : String -> String
doInline text = go (unpack text) "" False False
  where
    go : List Char -> String -> Bool -> Bool -> String
    go [] acc _ _ = acc
    go ('*' :: '*' :: rest) acc False iIt = go rest (acc ++ "<strong>") True iIt
    go ('*' :: '*' :: rest) acc True iIt = go rest (acc ++ "</strong>") False iIt
    go ('*' :: rest) acc iBold False = go rest (acc ++ "<em>") iBold True
    go ('*' :: rest) acc iBold True = go rest (acc ++ "</em>") iBold False
    go ('`' :: rest) acc iBold iIt =
      let (code, rem) = span (/= '`') rest
      in case rem of
           ('`' :: r) => go r (acc ++ "<code>" ++ pack code ++ "</code>") iBold iIt
           _ => go rest (acc ++ "`") iBold iIt
    go (c :: rest) acc iBold iIt = go rest (acc ++ singleton c) iBold iIt

doLine : String -> ParserState -> ParserState
doLine line st =
  let tr = trim line
  in if strHasPrefix "```" tr
       then if st.inCode
               then { html $= (++ "</code></pre>\n"), inCode := False } st
               else let st1 = closeUl (closePara st)
                    in { html $= (++ "<pre><code>"), inCode := True } st1
     else if st.inCode
       then { html $= (++ strEscape line ++ "\n") } st
     else if tr == ""
       then closeUl (closePara st)
     else if strHasPrefix "######" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "######" tr))
            in { html $= (++ "<h6>" ++ c ++ "</h6>\n") } st1
     else if strHasPrefix "#####" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "#####" tr))
            in { html $= (++ "<h5>" ++ c ++ "</h5>\n") } st1
     else if strHasPrefix "####" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "####" tr))
            in { html $= (++ "<h4>" ++ c ++ "</h4>\n") } st1
     else if strHasPrefix "###" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "###" tr))
            in { html $= (++ "<h3>" ++ c ++ "</h3>\n") } st1
     else if strHasPrefix "##" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "##" tr))
            in { html $= (++ "<h2>" ++ c ++ "</h2>\n") } st1
     else if strHasPrefix "#" tr
       then let st1 = closeUl (closePara st)
                c = doInline (trim (strDropPrefix "#" tr))
            in { html $= (++ "<h1>" ++ c ++ "</h1>\n") } st1
     else if strHasPrefix "- " tr || strHasPrefix "* " tr
       then let st1 = closePara st
                st2 = if st1.inList && st1.orderedList then closeUl st1 else st1
                st3 = if not st2.inList
                         then { html $= (++ "<ul>\n"), inList := True, orderedList := False } st2
                         else st2
                item = doInline (trim (substr 2 (length tr) tr))
            in { html $= (++ "<li>" ++ item ++ "</li>\n") } st3
     else let st1 = if not st.inPara
                       then { html $= (++ "<p>"), inPara := True } st
                       else { html $= (++ " ") } st
          in { html $= (++ doInline tr) } st1

export
parseMarkdown : String -> String
parseMarkdown content =
  let allLines = lines content
      final = foldl (flip doLine) initState allLines
      st1 = closeUl (closePara final)
      st2 = if st1.inCode then { html $= (++ "</code></pre>\n") } st1 else st1
  in st2.html

-- ============================================================================
-- Template Engine
-- ============================================================================

defTemplate : String
defTemplate = "<!DOCTYPE html>\n<html><head><meta charset=\"UTF-8\"><title>{{title}}</title><style>body{font-family:system-ui;max-width:800px;margin:0 auto;padding:2rem}pre{background:#f4f4f4;padding:1rem}</style></head><body><article><h1>{{title}}</h1><time>{{date}}</time>{{content}}</article></body></html>"

strReplace : String -> String -> String -> String
strReplace hay needle rep =
  if needle == "" then hay
  else go hay ""
  where
    go : String -> String -> String
    go str acc =
      if length str < length needle
         then acc ++ str
         else if strHasPrefix needle str
                 then go (substr (length needle) (length str) str) (acc ++ rep)
                 else go (substr 1 (length str) str) (acc ++ substr 0 1 str)

export
applyTemplate : Frontmatter -> String -> String
applyTemplate fm htmlContent =
  let t1 = strReplace defTemplate "{{title}}" fm.title
      t2 = strReplace t1 "{{date}}" fm.date
      t3 = strReplace t2 "{{content}}" htmlContent
  in t3

-- ============================================================================
-- Tests
-- ============================================================================

testMarkdown : IO ()
testMarkdown = do
  putStrLn "=== Test: Markdown ==="
  let md = "# Hello World\n\nThis is a **bold** test with *italic* text.\n\n- Item 1\n- Item 2\n\n```\ncode block\n```\n"
  putStrLn (parseMarkdown md)

testFrontmatter : IO ()
testFrontmatter = do
  putStrLn "=== Test: Frontmatter ==="
  let content = "---\ntitle: My Post\ndate: 2024-01-15\ntags: [idris, ssg]\ndraft: false\n---\n\nContent here\n"
  let (fm, body) = parseFrontmatter content
  putStrLn ("Title: " ++ fm.title)
  putStrLn ("Date: " ++ fm.date)
  putStrLn ("Tags: " ++ show fm.tags)
  putStrLn ("Draft: " ++ show fm.draft)
  putStrLn ("Body: " ++ body)

testFull : IO ()
testFull = do
  putStrLn "=== Test: Full Pipeline ==="
  let content = "---\ntitle: Welcome\ndate: 2024-01-15\n---\n\n# Welcome\n\nThis is **Ddraig**, an Idris 2-powered SSG.\n\n- Dependently typed\n- Provably correct\n- Elegant\n"
  let (fm, body) = parseFrontmatter content
  let htmlOut = parseMarkdown body
  let output = applyTemplate fm htmlOut
  putStrLn output

-- ============================================================================
-- Main
-- ============================================================================

main : IO ()
main = do
  args <- getArgs
  case args of
    [_, "test-markdown"] => testMarkdown
    [_, "test-frontmatter"] => testFrontmatter
    [_, "test-full"] => testFull
    _ => do
      putStrLn "Ddraig SSG - Idris 2 powered"
      putStrLn "Commands: test-markdown test-frontmatter test-full"
