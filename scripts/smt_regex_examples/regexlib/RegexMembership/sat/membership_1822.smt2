(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 5 5) (re.range "0" "9"))(re.++ (re.opt (re.++ (re.range "-" "-") ((_ re.loop 3 3) (re.range "0" "9")))) (str.to_re "")))))
