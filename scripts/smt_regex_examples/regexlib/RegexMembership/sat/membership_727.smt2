(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ (re.union (str.to_re (seq.++ "S" (seq.++ "i" (seq.++ "r" ""))))(re.union (re.++ (str.to_re (seq.++ "D" (seq.++ "r" ""))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.union (re.++ (str.to_re (seq.++ "M" (seq.++ "r" ""))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.union (re.++ (str.to_re (seq.++ "M" (seq.++ "r" (seq.++ "s" "")))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.union (re.++ (str.to_re (seq.++ "M" (seq.++ "s" ""))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))) (re.++ (str.to_re (seq.++ "R" (seq.++ "e" (seq.++ "v" "")))) (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))))))) (re.opt (re.range " " " "))))(re.++ (re.union (re.++ (re.range "A" "Z")(re.++ (re.range "." ".") (re.opt (re.++ (re.range "A" "Z") (re.range "." ".")))))(re.union (re.++ (re.range "A" "Z") (re.+ (re.range "a" "z")))(re.union (re.++ (re.range "A" "Z")(re.++ (re.+ (re.range "a" "z"))(re.++ (re.range "-" "-")(re.++ (re.range "A" "Z") (re.+ (re.range "a" "z")))))) (re.++ (re.range "A" "Z")(re.++ (re.* (re.range "a" "z"))(re.++ (re.range " " " ")(re.++ (re.range "A" "Z") (re.* (re.range "a" "z"))))))))) (str.to_re "")))))
