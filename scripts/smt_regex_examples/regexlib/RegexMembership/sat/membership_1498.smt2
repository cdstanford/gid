(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "1"))(re.++ (re.opt (re.union (re.range " " " ") (re.range "-" ".")))(re.++ (re.opt (re.range "(" "("))(re.++ (re.range "2" "9")(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.opt (re.range ")" ")"))(re.++ (re.opt (re.union (re.range " " " ") (re.range "-" ".")))(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.opt (re.union (re.range " " " ") (re.range "-" ".")))(re.++ ((_ re.loop 4 4) (re.range "0" "9")) (str.to_re "")))))))))))))
