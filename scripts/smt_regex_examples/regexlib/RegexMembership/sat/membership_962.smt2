(explore-derivatives (re.++ ((_ re.loop 5 5) (re.union (re.range "0" "9") (re.range "A" "Z")))(re.++ (re.range "0" "9")(re.++ (re.union (re.range "0" "1") (re.range "5" "6"))(re.++ (re.range "0" "9")(re.++ (re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (re.++ (re.range "3" "3") (re.range "0" "1"))))(re.++ (re.range "0" "9")(re.++ ((_ re.loop 3 3) (re.union (re.range "0" "9") (re.range "A" "Z"))) ((_ re.loop 2 2) (re.range "A" "Z"))))))))))
