(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.range "0" "9"))(re.++ (re.range "0" "9")(re.++ (re.opt (re.union (str.to_re "")(re.union (re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))(re.++ (re.* (re.range "0" "9")) (re.range "0" "9"))) (re.++ (re.range "," ",")(re.++ (re.* (re.range "0" "9")) (re.range "0" "9")))))) (str.to_re ""))))))
