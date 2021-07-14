(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "F" "G") (re.range "S" "T"))(re.++ ((_ re.loop 7 7) (re.range "0" "9"))(re.++ (re.range "A" "Z") (str.to_re ""))))))
