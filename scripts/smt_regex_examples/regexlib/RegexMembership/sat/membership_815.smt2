(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 0 5) (re.range "0" "9"))(re.++ (re.opt (re.range " " " "))(re.++ ((_ re.loop 0 6) (re.range "0" "9")) (str.to_re ""))))))
