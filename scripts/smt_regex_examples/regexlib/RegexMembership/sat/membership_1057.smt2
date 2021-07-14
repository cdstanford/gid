(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.opt (re.range "1" "2")) (re.range "0" "9")) (re.++ (re.range "3" "3") (re.range "0" "1"))) (str.to_re ""))))
