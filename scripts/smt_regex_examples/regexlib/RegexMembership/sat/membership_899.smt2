(explore-derivatives (re.union (re.++ (re.* (re.range "\x00" "\xff"))(re.++ (re.range "<" "<") (re.* (re.range "\x00" "\xff")))) (re.++ (re.range "/" "/")(re.++ (re.range "\x00" "\xff")(re.++ (str.to_re (seq.++ "]" (seq.++ ">" ""))) (re.* (re.range "\x00" "\xff")))))))
