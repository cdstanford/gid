(explore-derivatives (re.union (re.++ (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))) (str.to_re (seq.++ "$" (seq.++ "A" (seq.++ "V" (seq.++ "E" "")))))) (re.++ (str.to_re (seq.++ "$" (seq.++ "a" (seq.++ "v" (seq.++ "e" ""))))) (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))))))
