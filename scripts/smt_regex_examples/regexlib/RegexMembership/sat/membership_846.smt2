(explore-derivatives (re.++ (re.union (str.to_re (seq.++ "1" (seq.++ "9" ""))) (str.to_re (seq.++ "2" (seq.++ "0" ""))))(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.range "/" "/")(re.++ ((_ re.loop 6 6) (re.range "0" "9"))(re.++ (re.range "/" "/") ((_ re.loop 2 2) (re.range "0" "9"))))))))
