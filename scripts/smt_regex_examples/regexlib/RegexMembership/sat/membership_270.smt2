(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.range "+" "+")(re.++ (re.++ ((_ re.loop 2 2) (re.range "0" "9")) (re.* (re.range "0" "9")))(re.++ (re.++ ((_ re.loop 4 4) (re.range "0" "9")) (re.* (re.range "0" "9"))) (re.* (re.range "0" "9")))))(re.++ (re.opt (re.++ (re.opt (re.range "x" "x")) (re.+ (re.range "0" "9")))) (str.to_re "")))))
