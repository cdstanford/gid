(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.++ (re.range "5" "5")(re.++ (re.range "1" "5") ((_ re.loop 2 2) (re.range "0" "9")))) ((_ re.loop 12 12) (re.range "0" "9")))) (re.++ (re.++ (re.range "4" "4") ((_ re.loop 3 3) (re.range "0" "9")))(re.++ (re.union ((_ re.loop 12 12) (re.range "0" "9")) ((_ re.loop 9 9) (re.range "0" "9"))) (str.to_re "")))))
