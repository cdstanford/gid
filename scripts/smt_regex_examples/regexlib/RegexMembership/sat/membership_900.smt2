(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.range "-" "-"))(re.++ (re.union (re.++ (re.opt (re.range "1" "1"))(re.++ (re.range "1" "7") (re.range "1" "9")))(re.union (re.++ (re.opt (re.range "1" "1"))(re.++ (re.range "1" "8") (re.range "0" "0"))) (re.++ (re.opt (re.range "1" "9")) (re.range "0" "9"))))(re.++ (re.range "." ".") ((_ re.loop 1 6) (re.range "0" "9")))))))
