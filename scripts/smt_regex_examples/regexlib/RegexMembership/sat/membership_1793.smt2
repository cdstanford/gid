(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "\x00" "&") (re.range "(" "\xff"))) (str.to_re ""))))
