(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (str.to_re (seq.++ "1" (seq.++ "9" (seq.++ "2" (seq.++ "." (seq.++ "1" (seq.++ "6" (seq.++ "8" (seq.++ "." ""))))))))) (re.range "0" "9"))(re.union (re.++ (re.range "1" "9") (re.range "0" "9"))(re.union (re.++ (re.range "1" "2")(re.++ (re.range "0" "5")(re.++ (re.range "0" "5")(re.++ (re.range "." ".") (re.range "0" "9")))))(re.union (re.++ (re.range "1" "9") (re.range "0" "9")) (re.++ (re.range "1" "2")(re.++ (re.range "0" "5") (re.range "0" "5"))))))) (str.to_re ""))))
