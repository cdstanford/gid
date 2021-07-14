(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.range "A" "z"))(re.++ ((_ re.loop 8 8) (re.range "0" "9"))(re.++ (re.range "A" "z") (str.to_re ""))))))
