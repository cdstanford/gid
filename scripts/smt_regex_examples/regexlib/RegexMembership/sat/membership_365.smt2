(explore-derivatives (re.++ (re.union (re.++ (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" ""))))) (re.opt (re.range "s" "s")))(re.union (str.to_re (seq.++ "f" (seq.++ "t" (seq.++ "p" ""))))(re.union (str.to_re (seq.++ "g" (seq.++ "o" (seq.++ "p" (seq.++ "h" (seq.++ "e" (seq.++ "r" "")))))))(re.union (str.to_re (seq.++ "t" (seq.++ "e" (seq.++ "l" (seq.++ "n" (seq.++ "e" (seq.++ "t" "")))))))(re.union (str.to_re (seq.++ "f" (seq.++ "i" (seq.++ "l" (seq.++ "e" "")))))(re.union (str.to_re (seq.++ "n" (seq.++ "o" (seq.++ "t" (seq.++ "e" (seq.++ "s" "")))))) (str.to_re (seq.++ "m" (seq.++ "s" (seq.++ "-" (seq.++ "h" (seq.++ "e" (seq.++ "l" (seq.++ "p" ""))))))))))))))(re.++ (re.range ":" ":")(re.++ (re.+ (re.union (str.to_re (seq.++ "/" (seq.++ "/" ""))) (str.to_re (seq.++ "\x5c" (seq.++ "\x5c" ""))))) (re.* (re.union (re.range "#" "&")(re.union (re.range "(" ")")(re.union (re.range "+" "=")(re.union (re.range "@" "Z")(re.union (re.range "\x5c" "\x5c")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "~" "~")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff")))))))))))))))))))
