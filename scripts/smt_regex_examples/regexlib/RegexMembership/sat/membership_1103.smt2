(explore-derivatives (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "H" (seq.++ "R" (seq.++ "-" ""))))(re.++ ((_ re.loop 5 5) (re.range "0" "9")) (str.to_re "")))))
