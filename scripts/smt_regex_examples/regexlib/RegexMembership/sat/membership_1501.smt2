(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "A" "Z")(re.++ (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))) (str.to_re "")))))
