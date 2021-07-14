(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "1" "9")(re.++ ((_ re.loop 0 2) (re.range "0" "9")) (str.to_re "")))))
