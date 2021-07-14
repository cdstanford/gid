(explore-derivatives (re.++ (str.to_re (seq.++ "&" (seq.++ "#" "")))(re.++ ((_ re.loop 2 5) (re.range "0" "9")) (re.range ";" ";"))))
