(explore-derivatives (re.++ (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" "")))))(re.++ (re.union (re.range "I" "I") (re.range "i" "i"))(re.++ (re.union (re.range "M" "M") (re.range "m" "m"))(re.++ (re.union (re.range "G" "G") (re.range "g" "g")) (re.++ (re.* (re.union (re.range "\x00" "%")(re.union (re.range "'" ":")(re.union (re.range "<" "f")(re.union (re.range "h" "s") (re.range "u" "\xff"))))))(re.++ (re.* (re.union (re.range "\x00" "%")(re.union (re.range "'" ".")(re.union (re.range "0" ":")(re.union (re.range "<" "f")(re.union (re.range "h" "s") (re.range "u" "\xff")))))))(re.++ (re.* (re.union (re.range "&" "&")(re.union (re.range "/" "/")(re.union (re.range ";" ";")(re.union (re.range "g" "g") (re.range "t" "t")))))) (re.union (re.range "&" "&")(re.union (re.range ";" ";")(re.union (re.range "g" "g") (re.range "t" "t"))))))))))))
