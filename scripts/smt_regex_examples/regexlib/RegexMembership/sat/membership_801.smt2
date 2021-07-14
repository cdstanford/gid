(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.++ (re.range "0" "9") (re.opt (re.range "," ","))))(re.++ (re.++ (re.range "0" "9") (re.range "0" "9")) (str.to_re "")))))
