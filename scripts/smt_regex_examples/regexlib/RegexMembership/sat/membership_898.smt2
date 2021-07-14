(explore-derivatives (re.++ (re.range "\x22" "\x22")(re.++ ((_ re.loop 3 3) (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (re.range "\x22" "\x22"))))
