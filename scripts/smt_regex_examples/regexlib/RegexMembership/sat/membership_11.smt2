(explore-derivatives (re.++ (re.union (re.++ (re.range "1" "1") (re.union (re.range "," ",") (re.range "8" "9"))) (str.to_re (seq.++ "2" (seq.++ "0" "")))) ((_ re.loop 2 2) (re.range "0" "9"))))
