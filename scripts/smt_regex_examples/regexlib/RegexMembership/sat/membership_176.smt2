(explore-derivatives (re.++ (re.range "\x22" "\x22")(re.++ (re.* (re.union (re.++ (re.range "\x5c" "\x5c") (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))) (re.union (re.range "\x00" "!")(re.union (re.range "#" "[") (re.range "]" "\xff"))))) (re.range "\x22" "\x22"))))
