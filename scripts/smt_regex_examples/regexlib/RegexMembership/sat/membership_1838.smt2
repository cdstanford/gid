(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.range "/" "/")(re.++ ((_ re.loop 2 4) (re.range "0" "9")) (str.to_re ""))))))
