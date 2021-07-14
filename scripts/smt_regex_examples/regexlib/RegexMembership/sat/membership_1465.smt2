(explore-derivatives (re.++ (str.to_re (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " ""))))))) ((_ re.loop 1 5) (re.union (re.range "0" "9") (re.range "A" "F")))))
