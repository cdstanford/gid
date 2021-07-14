(explore-derivatives (re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (str.to_re (seq.++ "d" (seq.++ " " "")))(re.++ ((_ re.loop 1 2) (re.range "0" "9")) (re.range "h" "h")))))
