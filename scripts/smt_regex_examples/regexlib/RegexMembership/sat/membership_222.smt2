(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.++ ((_ re.loop 3 3) (re.range "0" "9")) (re.opt (re.range "-" "-"))))(re.++ ((_ re.loop 6 7) (re.range "0" "9")) (str.to_re "")))))
