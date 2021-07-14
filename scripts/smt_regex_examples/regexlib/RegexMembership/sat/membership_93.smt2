(explore-derivatives (re.++ (str.to_re "")(re.++ (re.+ (re.range "9" "9"))(re.++ (re.+ (re.union (re.range "1" "3") (re.range "6" "6")))(re.++ ((_ re.loop 7 7) (re.range "0" "9")) (str.to_re ""))))))
