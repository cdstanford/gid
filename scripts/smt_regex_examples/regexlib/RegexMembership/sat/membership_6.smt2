(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "2" (seq.++ "0" "")))(re.union (str.to_re (seq.++ "2" (seq.++ "3" "")))(re.union (str.to_re (seq.++ "2" (seq.++ "7" "")))(re.union (str.to_re (seq.++ "3" (seq.++ "0" ""))) (str.to_re (seq.++ "3" (seq.++ "3" "")))))))(re.++ (re.range "-" "-")(re.++ ((_ re.loop 8 8) (re.range "0" "9"))(re.++ (re.range "-" "-")(re.++ (re.range "0" "9") (str.to_re ""))))))))
