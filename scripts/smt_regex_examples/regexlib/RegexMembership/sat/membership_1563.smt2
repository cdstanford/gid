(explore-derivatives (re.++ (re.opt (re.++ (re.opt (re.range "(" "("))(re.++ ((_ re.loop 3 3) (re.range "0" "9")) (re.opt (re.range ")" ")")))))(re.++ (re.opt (re.union (re.range " " " ") (re.range "-" ".")))(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.union (re.range " " " ") (re.range "-" ".")) ((_ re.loop 4 4) (re.range "0" "9")))))))
