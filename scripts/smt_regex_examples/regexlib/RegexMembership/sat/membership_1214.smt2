(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.range "1" "9"))(re.++ (re.range "0" "9")(re.++ (re.opt (re.++ (re.range "." ".") (re.range "0" "9"))) (str.to_re ""))))))
