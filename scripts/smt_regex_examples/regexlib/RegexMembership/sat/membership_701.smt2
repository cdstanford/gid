(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "9" "9")(re.++ (re.union (re.range "2" "4") (re.range "7" "9"))(re.++ ((_ re.loop 8 8) (re.range "0" "9")) (str.to_re ""))))))
