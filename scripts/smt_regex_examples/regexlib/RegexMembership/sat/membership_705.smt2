(explore-derivatives (re.++ (re.opt (re.range "." "."))(re.++ (re.+ (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (str.to_re ""))))
