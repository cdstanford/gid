(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.union (re.range "1" "9")(re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (str.to_re (seq.++ "3" (seq.++ "0" ""))))))(re.++ (re.range "-" "-") (re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "A" "A") (re.range "a" "a")))(re.++ (re.union (re.range "," ",")(re.union (re.range "P" "P") (re.range "p" "p"))) (re.union (re.range "," ",")(re.union (re.range "R" "R") (re.range "r" "r")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "J" "J") (re.range "j" "j")))(re.++ (re.union (re.range "," ",")(re.union (re.range "U" "U") (re.range "u" "u"))) (re.union (re.range "," ",")(re.union (re.range "N" "N") (re.range "n" "n")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "S" "S") (re.range "s" "s")))(re.++ (re.union (re.range "," ",")(re.union (re.range "E" "E") (re.range "e" "e"))) (re.union (re.range "," ",")(re.union (re.range "P" "P") (re.range "p" "p"))))) (re.++ (re.union (re.range "," ",")(re.union (re.range "N" "N") (re.range "n" "n")))(re.++ (re.union (re.range "," ",")(re.union (re.range "O" "O") (re.range "o" "o"))) (re.union (re.range "," ",")(re.union (re.range "V" "V") (re.range "v" "v")))))))))) (re.++ (re.union (re.range "1" "9")(re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (re.++ (re.range "3" "3") (re.range "0" "1")))))(re.++ (re.range "-" "-") (re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "J" "J") (re.range "j" "j")))(re.++ (re.union (re.range "," ",")(re.union (re.range "A" "A") (re.range "a" "a"))) (re.union (re.range "," ",")(re.union (re.range "N" "N") (re.range "n" "n")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "M" "M") (re.range "m" "m")))(re.++ (re.union (re.range "," ",")(re.union (re.range "A" "A") (re.range "a" "a"))) (re.union (re.range "," ",")(re.union (re.range "R" "R") (re.range "r" "r")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "M" "M") (re.range "m" "m")))(re.++ (re.union (re.range "," ",")(re.union (re.range "A" "A") (re.range "a" "a"))) (re.union (re.range "," ",")(re.union (re.range "Y" "Y") (re.range "y" "y")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "J" "J") (re.range "j" "j")))(re.++ (re.union (re.range "," ",")(re.union (re.range "U" "U") (re.range "u" "u"))) (re.union (re.range "," ",")(re.union (re.range "L" "L") (re.range "l" "l")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "A" "A") (re.range "a" "a")))(re.++ (re.union (re.range "," ",")(re.union (re.range "U" "U") (re.range "u" "u"))) (re.union (re.range "," ",")(re.union (re.range "G" "G") (re.range "g" "g")))))(re.union (re.++ (re.union (re.range "," ",")(re.union (re.range "O" "O") (re.range "o" "o")))(re.++ (re.union (re.range "," ",")(re.union (re.range "C" "C") (re.range "c" "c"))) (re.union (re.range "," ",")(re.union (re.range "T" "T") (re.range "t" "t"))))) (re.++ (re.union (re.range "," ",")(re.union (re.range "D" "D") (re.range "d" "d")))(re.++ (re.union (re.range "," ",")(re.union (re.range "E" "E") (re.range "e" "e"))) (re.union (re.range "," ",")(re.union (re.range "C" "C") (re.range "c" "c"))))))))))))))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 4 4) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (re.union (re.range "1" "9")(re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.++ (re.range "2" "2") (re.range "0" "8")))))(re.++ (re.range "-" "-")(re.++ (re.++ (re.union (re.range "," ",")(re.union (re.range "F" "F") (re.range "f" "f")))(re.++ (re.union (re.range "," ",")(re.union (re.range "E" "E") (re.range "e" "e"))) (re.union (re.range "," ",")(re.union (re.range "B" "B") (re.range "b" "b")))))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.union (re.++ (re.union (re.range "0" "0")(re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8"))))) (re.union (re.range "1" "3")(re.union (re.range "5" "7") (re.range "9" "9")))) (re.++ (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9"))))) (re.union (re.range "0" "1")(re.union (re.range "3" "5") (re.range "7" "9"))))) (str.to_re "")))))))) (re.++ (str.to_re "")(re.++ (re.union (re.range "1" "9")(re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.++ (re.range "2" "2") (re.range "0" "9")))))(re.++ (re.range "-" "-")(re.++ (re.++ (re.union (re.range "," ",")(re.union (re.range "F" "F") (re.range "f" "f")))(re.++ (re.union (re.range "," ",")(re.union (re.range "E" "E") (re.range "e" "e"))) (re.union (re.range "," ",")(re.union (re.range "B" "B") (re.range "b" "b")))))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.union (re.++ (re.union (re.range "0" "0")(re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8"))))) (re.union (re.range "0" "0")(re.union (re.range "4" "4") (re.range "8" "8")))) (re.++ (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9"))))) (re.union (re.range "2" "2") (re.range "6" "6")))) (str.to_re "")))))))))))
