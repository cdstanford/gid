(explore-derivatives (re.++ (re.opt (re.union (re.++ (str.to_re (seq.++ "+" (seq.++ "9" (seq.++ "1" "")))) (re.opt (re.range "-" "-")))(re.union (re.++ (str.to_re (seq.++ "9" (seq.++ "1" ""))) (re.opt (re.range "-" "-"))) (re.++ (re.range "0" "0") (re.opt (re.range "-" "-"))))))(re.++ (re.range "9" "9") ((_ re.loop 9 9) (re.range "0" "9")))))
