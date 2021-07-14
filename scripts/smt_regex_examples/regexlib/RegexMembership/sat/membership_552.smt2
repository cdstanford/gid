(explore-derivatives (re.++ (str.to_re "")(re.++ (re.+ (re.++ ((_ re.loop 4 4) (re.range "0" "9")) (re.opt (re.range "," ",")))) (str.to_re ""))))
