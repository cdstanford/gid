(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (str.to_re (seq.++ "1" (seq.++ "8" "")))(re.++ (re.range "5" "9") (re.range "0" "9")))(re.union (re.++ (re.union (str.to_re (seq.++ "1" (seq.++ "9" ""))) (str.to_re (seq.++ "2" (seq.++ "0" "")))) ((_ re.loop 2 2) (re.range "0" "9"))) (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "0" (seq.++ "0" ""))))))) (str.to_re ""))))
