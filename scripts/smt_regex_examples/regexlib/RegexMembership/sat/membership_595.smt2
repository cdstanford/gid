(explore-derivatives (re.union (re.++ (re.+ (re.range "0" "0")) (re.++ (re.range "." ".")(re.++ (re.range "1" "9") (re.opt (re.range "0" "9")))))(re.union (re.++ (re.+ (re.range "0" "0")) (re.++ (re.range "." ".")(re.++ (re.range "0" "9") (re.+ (re.range "1" "9")))))(re.union (re.++ (re.+ (re.++ (re.+ (re.range "1" "9")) (re.opt (re.range "0" "0")))) (re.opt (re.++ (re.range "." ".") (re.+ (re.range "0" "9"))))) (re.++ (re.+ (re.++ (re.+ (re.range "1" "9")) (re.opt (re.range "0" "0")))) (re.opt (re.++ (re.range "." ".") (re.+ (re.range "0" "9")))))))))
