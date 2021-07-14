(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "," ",") (re.range "0" "9")))(re.++ (re.opt (re.range "." "."))(re.++ (re.* (re.range "0" "9")) (str.to_re ""))))))
