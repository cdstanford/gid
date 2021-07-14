(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "A" "Z")(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.opt (re.++ (re.range "." ".") (re.range "0" "9"))) (str.to_re ""))))))
