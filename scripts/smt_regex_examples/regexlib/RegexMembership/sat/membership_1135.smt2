(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (str.to_re (seq.++ "6" (seq.++ "5" (seq.++ "5" (seq.++ "3" ""))))) (re.range "0" "5"))(re.union (re.++ (str.to_re (seq.++ "6" (seq.++ "5" (seq.++ "5" ""))))(re.++ (re.range "0" "2") (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "6" (seq.++ "5" "")))(re.++ (re.range "0" "4")(re.++ (re.range "0" "9") (re.range "0" "9"))))(re.union (re.++ (re.range "6" "6")(re.++ (re.range "0" "4") ((_ re.loop 3 3) (re.range "0" "9"))))(re.union (re.++ (re.range "1" "5") ((_ re.loop 4 4) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 0 3) (re.range "0" "9"))) (re.range "0" "0"))))))) (str.to_re ""))))
