(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.union (re.++ (re.opt (re.range "0" "0")) (re.range "1" "9")) (re.++ (re.range "1" "1") (re.range "0" "2")))(re.++ (re.range ":" ":") (re.++ (re.range "0" "5") (re.range "0" "9")))) (str.to_re ""))))
