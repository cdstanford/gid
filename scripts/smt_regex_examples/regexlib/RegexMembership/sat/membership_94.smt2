(explore-derivatives (re.++ (str.to_re "")(re.++ (re.+ (re.range "0" "9"))(re.++ (re.opt (re.union (re.range "\x00" "+")(re.union (re.range "-" "-") (re.range "/" "\xff")))) (str.to_re "")))))
