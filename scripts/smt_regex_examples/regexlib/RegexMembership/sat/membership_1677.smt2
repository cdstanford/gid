(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 1 255) (re.union (re.range "." ".")(re.union (re.range "0" "9")(re.union (re.range "@" "Z") (re.range "a" "z"))))) (str.to_re ""))))
