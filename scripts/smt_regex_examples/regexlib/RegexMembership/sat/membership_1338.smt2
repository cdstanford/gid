(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.range "1" "9")(re.++ ((_ re.loop 0 2) (re.range "0" "9"))(re.++ (re.* (re.++ (re.range "," ",") ((_ re.loop 3 3) (re.range "0" "9")))) (re.opt (re.++ (re.range "." ".") ((_ re.loop 0 2) (re.range "0" "9")))))))(re.union (re.++ (re.range "1" "9")(re.++ (re.* (re.range "0" "9")) (re.opt (re.++ (re.range "." ".") ((_ re.loop 0 2) (re.range "0" "9"))))))(re.union (re.++ (re.range "0" "0") (re.opt (re.++ (re.range "." ".") ((_ re.loop 0 2) (re.range "0" "9"))))) (re.opt (re.++ (re.range "." ".") ((_ re.loop 1 2) (re.range "0" "9"))))))) (str.to_re ""))))
