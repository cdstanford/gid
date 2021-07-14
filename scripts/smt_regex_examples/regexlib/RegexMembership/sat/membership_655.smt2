(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 4 4) (re.range "A" "Z"))(re.++ (re.range "1" "8")(re.++ ((_ re.loop 2 2) (re.range "0" "9")) (str.to_re ""))))))
