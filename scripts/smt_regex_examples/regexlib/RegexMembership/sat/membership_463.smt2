(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" (seq.++ "9" (seq.++ "6" (seq.++ "7" (seq.++ "2" (seq.++ "9" "")))))))))) (re.range "0" "6"))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" (seq.++ "9" (seq.++ "6" (seq.++ "7" (seq.++ "2" "")))))))))(re.++ (re.range "0" "8") (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" (seq.++ "9" (seq.++ "6" (seq.++ "7" ""))))))))(re.++ (re.range "0" "1") ((_ re.loop 2 2) (re.range "0" "9"))))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" (seq.++ "9" (seq.++ "6" "")))))))(re.++ (re.range "0" "6") ((_ re.loop 3 3) (re.range "0" "9"))))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" (seq.++ "9" ""))))))(re.++ (re.range "0" "5") ((_ re.loop 4 4) (re.range "0" "9"))))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" (seq.++ "4" "")))))(re.++ (re.range "0" "8") ((_ re.loop 5 5) (re.range "0" "9"))))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" (seq.++ "9" ""))))(re.++ (re.range "0" "3") ((_ re.loop 6 6) (re.range "0" "9"))))(re.union (re.++ (str.to_re (seq.++ "4" (seq.++ "2" "")))(re.++ (re.range "0" "8") ((_ re.loop 7 7) (re.range "0" "9"))))(re.union (re.++ (re.range "4" "4")(re.++ (re.range "0" "1") ((_ re.loop 8 8) (re.range "0" "9"))))(re.union (re.++ (re.range "1" "3") ((_ re.loop 9 9) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 8 8) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 7 7) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 6 6) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 5 5) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 4 4) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 3 3) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") ((_ re.loop 2 2) (re.range "0" "9")))(re.union (re.++ (re.range "1" "9") (re.range "0" "9")) (re.range "0" "9"))))))))))))))))))) (str.to_re ""))))
