(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 9 9) (re.range "0" "9"))(re.++ (re.union (re.range "0" "9")(re.union (re.range "X" "X") (re.range "|" "|"))) (str.to_re "")))))
