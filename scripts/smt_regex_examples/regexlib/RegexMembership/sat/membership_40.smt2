(explore-derivatives (re.++ (str.to_re "")(re.++ (re.++ (re.range "+" "+") (str.to_re (seq.++ "9" (seq.++ "1" ""))))(re.++ (re.range "1" "9")(re.++ ((_ re.loop 9 9) (re.range "0" "9")) (str.to_re ""))))))
