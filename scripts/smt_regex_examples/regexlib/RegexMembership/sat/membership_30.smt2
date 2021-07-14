(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "\x00" "\x1f")(re.union (re.range "!" "+")(re.union (re.range "-" "/") (re.range "1" "\xff"))))) (str.to_re ""))))
