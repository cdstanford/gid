(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 1 20) (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (str.to_re ""))))
