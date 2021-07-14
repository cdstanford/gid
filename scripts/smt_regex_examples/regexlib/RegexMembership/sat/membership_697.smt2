(explore-derivatives (re.++ (re.range "&" "&")(re.++ (re.+ (re.union (re.range "A" "Z") (re.range "a" "z")))(re.++ ((_ re.loop 0 3) (re.range "0" "9")) (re.range ";" ";")))))
