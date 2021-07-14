(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 2 2) (re.union (re.range "1" "5") (re.range "|" "|")))(re.++ ((_ re.loop 14 14) (re.range "0" "9")) (str.to_re "")))))
