(explore-derivatives (re.++ (str.to_re "")(re.++ (re.+ (re.union (re.range "\x00" "/")(re.union (re.range ":" "@")(re.union (re.range "[" "`") (re.range "{" "\xff"))))) (str.to_re ""))))
