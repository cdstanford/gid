(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "1" "9")(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.opt (re.range " " " "))(re.++ (re.union ((_ re.loop 2 2) (re.union (re.range "A" "R")(re.union (re.range "T" "Z")(re.union (re.range "a" "r") (re.range "t" "z"))))) (re.++ (re.union (re.range "S" "S") (re.range "s" "s")) (re.union (re.range "\x00" "@")(re.union (re.range "B" "C")(re.union (re.range "E" "R")(re.union (re.range "T" "`")(re.union (re.range "b" "c")(re.union (re.range "e" "r") (re.range "t" "\xff"))))))))) (str.to_re "")))))))
