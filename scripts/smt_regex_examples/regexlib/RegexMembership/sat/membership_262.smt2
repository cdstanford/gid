(explore-derivatives (re.++ (re.+ (re.range "(" "(")) (re.++ (re.* (re.union (re.range "\x00" "'") (re.range ")" "\xff"))) (re.+ (re.range ")" ")")))))
