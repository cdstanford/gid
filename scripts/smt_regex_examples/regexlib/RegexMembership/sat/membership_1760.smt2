(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union ((_ re.loop 2 2) (re.range "1" "9"))(re.union (re.++ (re.range "0" "9") (re.range "1" "9")) (re.++ (re.range "1" "9") (re.range "0" "9"))))(re.++ ((_ re.loop 3 3) (re.range "0" "9")) (str.to_re "")))))
