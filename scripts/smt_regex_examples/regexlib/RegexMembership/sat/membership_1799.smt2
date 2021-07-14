(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 0 2) (re.range "0" "9"))(re.++ (re.opt (re.++ (re.range "." ".") ((_ re.loop 1 2) (re.range "0" "9")))) (str.to_re "")))))
