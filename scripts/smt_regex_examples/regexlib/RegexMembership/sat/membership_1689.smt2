(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.union (re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (re.++ (re.range "3" "3") (re.range "0" "1")))) (re.range "1" "9"))(re.++ (re.range "/" "/")(re.++ (re.union (re.union (re.++ (re.range "0" "0") (re.range "1" "9")) (re.++ (re.range "1" "1") (re.range "0" "2"))) (re.range "1" "9"))(re.++ (re.range "/" "/") (re.union ((_ re.loop 2 2) (re.range "0" "9")) (re.++ (re.union (str.to_re (seq.++ "1" (seq.++ "9" ""))) (re.++ (re.range "2" "2") (re.range "0" "0"))) ((_ re.loop 2 2) (re.range "0" "9")))))))) (str.to_re ""))))
