(explore-derivatives (re.++ (re.opt (str.to_re (seq.++ "+" (seq.++ "1" (seq.++ " " "")))))(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.range " " " ")(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.range " " " ")(re.++ ((_ re.loop 4 4) (re.range "0" "9")) (re.range " " " "))))))))
