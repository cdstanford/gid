(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "2" "")))) (re.range "0" "3"))(re.union (re.++ (str.to_re (seq.++ "1" (seq.++ "0" "")))(re.++ (re.range "0" "1") (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 0 2) (re.range "0" "9"))) (re.range "0" "0")))) (str.to_re ""))))
