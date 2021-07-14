(explore-derivatives (re.++ (re.range "{" "{")(re.++ (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))) (re.range "}" "}"))))
