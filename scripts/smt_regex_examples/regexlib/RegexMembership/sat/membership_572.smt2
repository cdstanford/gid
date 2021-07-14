(explore-derivatives (re.++ (re.range "0" "9")(re.++ (re.range "." ".")(re.++ ((_ re.loop 3 3) (re.range "0" "9")) (str.to_re "")))))
