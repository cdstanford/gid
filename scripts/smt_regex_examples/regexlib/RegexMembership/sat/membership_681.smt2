(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ ((_ re.loop 2 4) (re.range "0" "9")) (re.range "/" "/")))(re.++ (re.union ((_ re.loop 6 8) (re.range "0" "9"))(re.union (re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 2 4) (re.range "0" "9")))))) (re.++ ((_ re.loop 3 4) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 3 4) (re.range "0" "9")))))) (str.to_re "")))))
