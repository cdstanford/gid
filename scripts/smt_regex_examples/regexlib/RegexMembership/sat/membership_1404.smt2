(explore-derivatives (re.union (re.++ (re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (re.++ (re.range "3" "3") (re.range "0" "1"))))(re.++ (re.range "/" "/")(re.++ (re.union (re.++ (re.range "0" "0") (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5") (re.range "7" "8")))))(re.union (str.to_re (seq.++ "1" (seq.++ "0" ""))) (str.to_re (seq.++ "1" (seq.++ "2" "")))))(re.++ (re.range "/" "/") (re.++ (re.range "1" "2")(re.++ (re.union (re.range "," ",")(re.union (re.range "0" "0") (re.range "9" "9")))(re.++ (re.range "0" "9") (re.range "0" "9"))))))))(re.union (re.++ (re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "2") (re.range "0" "9")) (str.to_re (seq.++ "3" (seq.++ "0" "")))))(re.++ (re.range "/" "/")(re.++ (re.union (re.++ (re.range "0" "0") (re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "9" "9")))) (str.to_re (seq.++ "1" (seq.++ "1" ""))))(re.++ (re.range "/" "/") (re.++ (re.range "1" "2")(re.++ (re.union (re.range "," ",")(re.union (re.range "0" "0") (re.range "9" "9")))(re.++ (re.range "0" "9") (re.range "0" "9"))))))))(re.union (re.++ (re.union (re.++ (re.range "0" "0") (re.range "1" "9"))(re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.++ (re.range "2" "2") (re.range "0" "8"))))(re.++ (re.range "/" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.range "1" "2")(re.++ (re.union (re.range "," ",")(re.union (re.range "0" "0") (re.range "9" "9")))(re.++ (re.range "0" "9") (re.range "0" "9"))))))))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "9" "")))(re.++ (re.range "-" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.union (re.range "0" "0")(re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8")))))(re.++ (re.union (re.range "0" "0")(re.union (re.range "4" "4") (re.range "8" "8"))) (str.to_re (seq.++ "0" (seq.++ "0" "")))))))))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "9" "")))(re.++ (re.range "/" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9")))))(re.++ (re.union (re.range "2" "2") (re.range "6" "6")) (str.to_re (seq.++ "0" (seq.++ "0" "")))))))))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "9" "")))(re.++ (re.range "/" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.range "0" "9")(re.++ (re.range "0" "9")(re.++ (re.range "0" "0") (re.union (re.range "4" "4") (re.range "8" "8")))))))))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "9" "")))(re.++ (re.range "/" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.range "0" "9")(re.++ (re.range "0" "9")(re.++ (re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8")))) (re.union (re.range "0" "0")(re.union (re.range "4" "4") (re.range "8" "8")))))))))) (re.++ (str.to_re (seq.++ "2" (seq.++ "9" "")))(re.++ (re.range "/" "/")(re.++ (str.to_re (seq.++ "0" (seq.++ "2" "")))(re.++ (re.range "/" "/") (re.++ (re.range "0" "9")(re.++ (re.range "0" "9")(re.++ (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9"))))) (re.union (re.range "2" "2") (re.range "6" "6")))))))))))))))))
