(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.opt (re.union (re.range "+" "+") (re.range "-" "-")))(re.++ (re.union (re.++ (re.+ (re.range "0" "9"))(re.++ (re.opt (re.range "." ".")) (re.* (re.range "0" "9")))) (re.++ (re.* (re.range "0" "9"))(re.++ (re.opt (re.range "." ".")) (re.+ (re.range "0" "9"))))) (re.opt (re.++ (re.union (re.range "E" "E") (re.range "e" "e"))(re.++ (re.opt (re.union (re.range "+" "+") (re.range "-" "-")))(re.++ (re.opt (re.range "0" "2")) ((_ re.loop 1 2) (re.range "0" "9")))))))) (str.to_re ""))))
