(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 4) (re.union (re.range "A" "Z") (re.range "a" "z")))(re.++ ((_ re.loop 6 6) (re.range "0" "9")) (str.to_re "")))))
