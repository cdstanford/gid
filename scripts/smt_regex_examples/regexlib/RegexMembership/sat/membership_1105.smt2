(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "B" "B")(re.union (re.range "K" "K")(re.union (re.range "P" "P")(re.union (re.range "T" "T") (re.range "|" "|")))))(re.++ (re.range "A" "Z")(re.++ ((_ re.loop 4 4) (re.range "0" "9")) (str.to_re ""))))))
