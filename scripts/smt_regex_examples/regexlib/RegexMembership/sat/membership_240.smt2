(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.opt (re.range "#" "#")) (re.union ((_ re.loop 6 6) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))) ((_ re.loop 3 3) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))))(re.union (re.++ (str.to_re (seq.++ "r" (seq.++ "g" (seq.++ "b" (seq.++ "(" "")))))(re.++ ((_ re.loop 2 2) (re.union (re.++ (re.range "0" "9") (re.range "," ","))(re.union (re.++ (re.range "1" "9")(re.++ (re.range "0" "9") (re.range "," ",")))(re.union (re.++ (re.range "1" "1")(re.++ ((_ re.loop 2 2) (re.range "0" "9")) (re.range "," ",")))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4")(re.++ (re.range "0" "9") (re.range "," ",")))) (re.++ (str.to_re (seq.++ "2" (seq.++ "5" "")))(re.++ (re.range "0" "5") (re.range "," ","))))))))(re.++ (re.union (re.range "0" "9")(re.union (re.++ (re.range "1" "9") (re.range "0" "9"))(re.union (re.++ (re.range "1" "1") ((_ re.loop 2 2) (re.range "0" "9")))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4") (re.range "0" "9"))) (re.++ (str.to_re (seq.++ "2" (seq.++ "5" ""))) (re.range "0" "5")))))) (re.range ")" ")")))) (re.++ (str.to_re (seq.++ "r" (seq.++ "g" (seq.++ "b" (seq.++ "(" "")))))(re.++ ((_ re.loop 2 2) (re.union (re.++ (re.range "0" "9") (str.to_re (seq.++ "%" (seq.++ "," ""))))(re.union (re.++ (re.range "1" "9")(re.++ (re.range "0" "9") (str.to_re (seq.++ "%" (seq.++ "," ""))))) (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "0" (seq.++ "%" (seq.++ "," "")))))))))(re.++ (re.union (re.++ (re.range "0" "9") (re.range "%" "%"))(re.union (re.++ (re.range "1" "9")(re.++ (re.range "0" "9") (re.range "%" "%"))) (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "0" (seq.++ "%" ""))))))) (re.range ")" ")")))))) (str.to_re ""))))
