(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union ((_ re.loop 3 3) (re.range "0" "9")) ((_ re.loop 4 4) (re.range "0" "9")))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 5 5) (re.range "0" "9")) (str.to_re ""))))))
