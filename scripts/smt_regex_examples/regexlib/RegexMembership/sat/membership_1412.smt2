(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 32 32) (re.union (re.range "0" "9") (re.range "a" "z"))) (str.to_re ""))))
