(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 5 5) (re.range "A" "Z"))(re.++ ((_ re.loop 4 4) (re.range "0" "9"))(re.++ (re.range "A" "Z") (str.to_re ""))))))
