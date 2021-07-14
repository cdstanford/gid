(explore-derivatives (re.++ (str.to_re (seq.++ "0" (seq.++ "x" ""))) (re.+ (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))))
