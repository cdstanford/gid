(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ ((_ re.loop 5 5) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 4 4) (re.range "0" "9"))))(re.union ((_ re.loop 5 5) (re.range "0" "9")) (re.++ (re.union (re.range "A" "C")(re.union (re.range "E" "E")(re.union (re.range "G" "H")(re.union (re.range "J" "N")(re.union (re.range "P" "P")(re.union (re.range "R" "T")(re.union (re.range "V" "V")(re.union (re.range "X" "Y")(re.union (re.range "a" "c")(re.union (re.range "e" "e")(re.union (re.range "g" "h")(re.union (re.range "j" "n")(re.union (re.range "p" "p")(re.union (re.range "r" "t")(re.union (re.range "v" "v") (re.range "x" "y"))))))))))))))))(re.++ (re.range "0" "9")(re.++ (re.union (re.range "A" "Z") (re.range "a" "z"))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "0" "9")(re.++ (re.union (re.range "A" "Z") (re.range "a" "z")) (re.range "0" "9"))))))))) (str.to_re ""))))
