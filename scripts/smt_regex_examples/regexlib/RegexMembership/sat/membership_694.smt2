(explore-derivatives (re.++ (re.range "<" "<")(re.++ (re.* (re.range "!" "!"))(re.++ (re.* (re.union (re.range "\x00" ";")(re.union (re.range "=" "=") (re.range "?" "\xff")))) (re.range ">" ">")))))
