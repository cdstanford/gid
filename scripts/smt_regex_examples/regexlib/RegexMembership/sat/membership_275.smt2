(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.union ((_ re.loop 8 8) (re.range "0" "9"))(re.union ((_ re.loop 10 10) (re.range "0" "9"))(re.union ((_ re.loop 11 11) (re.range "0" "9")) (re.++ ((_ re.loop 6 6) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 5 5) (re.range "0" "9")))))))) (str.to_re ""))))
