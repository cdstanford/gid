(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.range "0" "9"))(re.++ ((_ re.loop 2 2) (re.range "1" "9")) (str.to_re "")))))
