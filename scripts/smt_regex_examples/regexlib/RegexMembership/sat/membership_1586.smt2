(explore-derivatives (re.++ (str.to_re (seq.++ "/" (seq.++ "*" "")))(re.++ (re.* (re.range "\x00" "\xff")) (str.to_re (seq.++ "*" (seq.++ "/" ""))))))
