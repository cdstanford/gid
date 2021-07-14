(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.+ (re.range "0" "0")) (re.range "1" "9")) (re.range "1" "9"))(re.++ (re.* (re.range "0" "9")) (str.to_re "")))))
