(explore-derivatives (re.++ (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.++ (re.union (re.range "P" "P") (re.range "p" "p"))(re.++ (str.to_re (seq.++ "e" (seq.++ "n" "")))(re.++ (re.union (re.range "1" "1")(re.union (re.range "I" "I") (re.range "i" "i")))(re.++ (re.union (re.range "$" "$") (re.range "s" "s")) (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))))))))
