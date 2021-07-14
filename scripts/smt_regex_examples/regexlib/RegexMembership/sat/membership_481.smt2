(explore-derivatives (re.++ (str.to_re (seq.++ "/" (seq.++ "*" "")))(re.++ (re.+ (re.union (re.range "\x00" ".") (re.range "0" "\xff"))) (re.range "/" "/"))))
