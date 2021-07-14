(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 5 5) (re.union (re.range "0" "9") (re.range "A" "Z"))) (str.to_re ""))))
