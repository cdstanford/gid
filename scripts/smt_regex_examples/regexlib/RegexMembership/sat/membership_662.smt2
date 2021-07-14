(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 3) (re.range "A" "Z"))(re.++ ((_ re.loop 8 12) (re.union (re.range "0" "9") (re.range "A" "Z"))) (str.to_re "")))))
