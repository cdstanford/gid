(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "1" "9")(re.++ (re.opt (re.range "0" "9"))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 7 7) (re.range "0" "9")) (str.to_re "")))))))
