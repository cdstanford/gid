(explore-derivatives (re.union (re.++ (str.to_re "") (re.++ (re.+ (re.range "0" "9"))(re.++ (re.+ (re.range "." ".")) (re.+ (re.range "0" "9"))))) (re.++ (re.range "0" "0") (str.to_re ""))))
