(explore-derivatives (re.++ (str.to_re "")(re.++ (re.+ (re.range "a" "z"))(re.++ (re.* (re.range "0" "9"))(re.++ (re.+ (re.range "a" "z")) (str.to_re ""))))))
