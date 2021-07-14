(explore-derivatives (re.++ (re.range "/" "/")(re.++ (re.+ (re.union (re.range "\x00" ".") (re.range "0" "\xff"))) (str.to_re ""))))
