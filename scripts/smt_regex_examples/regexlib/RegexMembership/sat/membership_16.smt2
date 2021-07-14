(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "0" "9")(re.++ (re.* (re.union (re.range "," ",") (re.range "0" "9")))(re.++ (re.range "0" "9") (str.to_re ""))))))
