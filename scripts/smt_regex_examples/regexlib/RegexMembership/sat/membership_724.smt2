(explore-derivatives (re.++ (re.range "\x22" "\x22")(re.++ (re.+ (re.union (re.range "\x00" "!") (re.range "#" "\xff"))) (str.to_re (seq.++ "\x22" (seq.++ " " ""))))))
