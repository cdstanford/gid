(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 7 7) (re.range "0" "9"))(re.++ (re.range "0" "6") (str.to_re "")))))))
