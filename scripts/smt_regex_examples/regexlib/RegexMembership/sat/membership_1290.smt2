(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))) (str.to_re ""))))
