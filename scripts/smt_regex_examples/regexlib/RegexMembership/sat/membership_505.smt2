(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union ((_ re.loop 1 8) (re.range "0" "9")) (re.++ ((_ re.loop 0 8) (re.range "0" "9"))(re.++ (re.range "." ".") ((_ re.loop 1 2) (re.range "0" "9"))))) (str.to_re ""))))
