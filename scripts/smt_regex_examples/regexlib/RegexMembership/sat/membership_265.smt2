(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "A" "Z")(re.++ (re.range "-" "-")(re.++ ((_ re.loop 7 7) (re.range "0" "9")) (str.to_re ""))))))
