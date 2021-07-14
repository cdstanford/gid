(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 4 4) (re.union (re.range "0" "1") (re.range "6" "6")))(re.++ ((_ re.loop 12 12) (re.range "0" "9")) (str.to_re "")))))
