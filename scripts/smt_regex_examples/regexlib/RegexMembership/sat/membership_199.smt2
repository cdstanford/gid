(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "2"))(re.++ (re.range "1" "9") (str.to_re "")))) (re.++ (str.to_re "")(re.++ (re.range "3" "3")(re.++ (re.range "0" "1") (str.to_re ""))))))
