(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "s" (seq.++ "i" (seq.++ "p" "")))) (str.to_re (seq.++ "s" (seq.++ "i" (seq.++ "p" (seq.++ "s" ""))))))(re.++ (re.range ":" ":")(re.++ (re.opt (re.range "+" "+")) (re.+ (re.union (re.range "%" "&")(re.union (re.range "," ".")(re.union (re.range "0" ";")(re.union (re.range "=" "=")(re.union (re.range "?" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "|" "|")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))))))))))))
