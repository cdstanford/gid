(explore-derivatives (re.++ (re.union (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" (seq.++ ":" (seq.++ "/" (seq.++ "/" "")))))))) (re.++ (str.to_re (seq.++ "w" (seq.++ "w" (seq.++ "w" "")))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))) (re.* (re.union (re.range "." "9")(re.union (re.range "A" "Z")(re.union (re.range "a" "z") (re.range "~" "~")))))))
