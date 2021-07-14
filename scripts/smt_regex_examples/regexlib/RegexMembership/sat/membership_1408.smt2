(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 2 2) (re.range "0" "9"))))) (str.to_re ""))))
