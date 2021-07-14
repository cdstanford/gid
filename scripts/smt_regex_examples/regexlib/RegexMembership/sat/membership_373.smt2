(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 3) (re.union (re.range "0" "9") (re.range "A" "z"))) (str.to_re ""))))
