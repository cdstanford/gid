(explore-derivatives (re.++ (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range ";" ";")(re.union (re.range "=" "=") (re.range "A" "Z")))))) (re.range ":" ":"))) (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))))
