(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.opt ((_ re.loop 3 3) (re.union (re.range "A" "Z") (re.range "a" "z")))) ((_ re.loop 4 4) (re.range "0" "9"))) (str.to_re ""))))
