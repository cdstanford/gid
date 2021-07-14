(explore-derivatives (re.++ (re.range "$" "$")(re.++ (re.++ (re.opt (re.range "0" "9")) (re.+ (re.union (re.range "A" "Z") (re.range "a" "z")))) (re.opt (re.range "0" "9")))))
