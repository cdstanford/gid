(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 2 2) (re.union (re.range "C" "C") (re.range "S" "S")))(re.++ ((_ re.loop 6 6) (re.range "0" "9")) (str.to_re "")))))
