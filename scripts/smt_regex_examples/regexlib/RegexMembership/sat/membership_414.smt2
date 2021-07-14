(explore-derivatives (re.++ (re.range "." ".")(re.++ ((_ re.loop 3 4) (re.union (re.range "," ",")(re.union (re.range "1" "9") (re.range "a" "z")))) (re.range "/" "/"))))
