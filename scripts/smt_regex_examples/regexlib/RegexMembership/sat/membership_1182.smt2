(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.* (re.range "0" "9"))(re.++ (re.opt (re.range "." "."))(re.++ (re.* (re.range "0" "9"))(re.++ (re.+ (re.range "0" "9"))(re.++ (re.* (re.range "0" "9")) (str.to_re ""))))))) (re.++ (str.to_re "")(re.++ (re.+ (re.range "0" "9"))(re.++ (re.* (re.range "0" "9"))(re.++ (re.range "." ".")(re.++ (re.* (re.range "0" "9")) (str.to_re ""))))))))
