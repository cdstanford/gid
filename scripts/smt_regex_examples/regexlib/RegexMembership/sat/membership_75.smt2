(explore-derivatives (re.++ (re.union (re.++ (re.union (re.range "N" "N")(re.union (re.range "S" "S")(re.union (re.range "n" "n") (re.range "s" "s")))) (re.union (re.range "A" "H")(re.union (re.range "J" "Z")(re.union (re.range "a" "h") (re.range "j" "z")))))(re.union (re.++ (re.union (re.range "O" "O")(re.union (re.range "T" "T")(re.union (re.range "o" "o") (re.range "t" "t")))) (re.union (re.range "A" "B")(re.union (re.range "F" "G")(re.union (re.range "L" "M")(re.union (re.range "Q" "R")(re.union (re.range "V" "W")(re.union (re.range "a" "b")(re.union (re.range "f" "g")(re.union (re.range "l" "m")(re.union (re.range "q" "r") (re.range "v" "w")))))))))))(re.union (re.++ (re.union (re.range "H" "H") (re.range "h" "h")) (re.union (re.range "L" "Z") (re.range "l" "z"))) (re.++ (re.union (re.range "J" "J") (re.range "j" "j")) (re.union (re.range "L" "M")(re.union (re.range "Q" "R")(re.union (re.range "V" "W")(re.union (re.range "l" "m")(re.union (re.range "q" "r") (re.range "v" "w"))))))))))(re.++ (re.opt ((_ re.loop 2 2) (re.range "0" "9"))) (re.union (re.union (re.range "A" "N")(re.union (re.range "P" "Z")(re.union (re.range "a" "n") (re.range "p" "z")))) (re.++ (re.opt ((_ re.loop 2 2) (re.range "0" "9")))(re.++ (re.opt ((_ re.loop 2 2) (re.range "0" "9")))(re.++ (re.opt ((_ re.loop 2 2) (re.range "0" "9"))) (re.opt ((_ re.loop 2 2) (re.range "0" "9"))))))))))
