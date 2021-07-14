(explore-derivatives (re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))(re.++ (re.range "{" "{")(re.++ (re.range "0" "9") (re.range "}" "}")))))
