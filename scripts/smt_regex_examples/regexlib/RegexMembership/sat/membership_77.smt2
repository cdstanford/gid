(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.union (re.range "0" "4") (re.range "6" "9")) ((_ re.loop 4 4) (re.range "0" "9"))) (str.to_re ""))))
