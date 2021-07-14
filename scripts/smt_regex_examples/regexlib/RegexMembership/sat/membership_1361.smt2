(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 3) (re.range "A" "Z"))(re.++ ((_ re.loop 8 8) (re.range "0" "9")) (str.to_re "")))))
