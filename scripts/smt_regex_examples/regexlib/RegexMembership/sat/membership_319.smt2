(explore-derivatives (re.++ (re.++ (str.to_re (seq.++ "[" (seq.++ "u" (seq.++ "r" (seq.++ "l" "")))))(re.++ (re.opt (re.range "=" "=")) (re.opt (re.range "\x22" "\x22"))))(re.++ (re.* (re.union (re.range "\x00" "!")(re.union (re.range "#" "\x5c") (re.range "^" "\xff"))))(re.++ (re.++ (re.opt (re.range "\x22" "\x22")) (re.range "]" "]"))(re.++ (re.* (re.union (re.range "\x00" "Z") (re.range "\x5c" "\xff"))) (str.to_re (seq.++ "[" (seq.++ "/" (seq.++ "u" (seq.++ "r" (seq.++ "l" (seq.++ "]" ""))))))))))))
