(explore-derivatives (re.++ (str.to_re (seq.++ "P" (seq.++ "a" (seq.++ "s" (seq.++ "s" (seq.++ "w" (seq.++ "o" (seq.++ "r" (seq.++ "d" (seq.++ "=" (seq.++ "&" (seq.++ "q" (seq.++ "u" (seq.++ "o" (seq.++ "t" (seq.++ ";" ""))))))))))))))))(re.++ (re.union (re.++ (re.range "{" "{")(re.++ (re.+ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.++ (re.range "}" "}")(re.++ (re.+ (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (re.* (re.range "=" "=")))))) (re.+ (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z"))))) (str.to_re (seq.++ "&" (seq.++ "q" (seq.++ "u" (seq.++ "o" (seq.++ "t" (seq.++ ";" ""))))))))))
