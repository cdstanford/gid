(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "0" (seq.++ "9" ""))) (re.range "9" "9"))(re.++ (re.range "1" "1")(re.++ (re.range "1" "9")(re.++ (re.range "\x5c" "\x5c")(re.++ ((_ re.loop 7 7) (re.range "d" "d")) (str.to_re ""))))))) (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "0" (seq.++ "9" ""))) (re.range "9" "9"))(re.++ (re.range "3" "3")(re.++ (re.union (re.range "1" "2") (re.range "4" "6"))(re.++ (re.range "\x5c" "\x5c")(re.++ ((_ re.loop 7 7) (re.range "d" "d")) (str.to_re "")))))))))
