(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.range "4" "4")(re.++ ((_ re.loop 12 12) (re.range "0" "9")) (str.to_re ""))))(re.union (re.++ (str.to_re "")(re.++ (re.range "4" "4")(re.++ (re.range "0" "8")(re.++ ((_ re.loop 14 14) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" "")))(re.++ (re.union (re.range "\x00" "/")(re.union (re.range "2" "2") (re.range "4" "\xff")))(re.++ ((_ re.loop 13 13) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "0" (seq.++ "3" (seq.++ "0" ""))))))(re.++ (re.range "0" "1")(re.++ ((_ re.loop 10 10) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "0" (seq.++ "3" (seq.++ "3" ""))))))(re.++ (re.range "0" "4")(re.++ ((_ re.loop 10 10) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "1" (seq.++ "1" (seq.++ "0" ""))))))(re.++ (re.union (re.range "\x00" "0") (re.range "3" "\xff"))(re.++ ((_ re.loop 10 10) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "1" (seq.++ "1" (seq.++ "7" ""))))))(re.++ (re.range "0" "3")(re.++ ((_ re.loop 10 10) (re.range "0" "9")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "1" (seq.++ "1" (seq.++ "8" ""))))))(re.++ (re.union (re.range "\x00" "/") (re.range "3" "\xff"))(re.++ ((_ re.loop 10 10) (re.range "0" "9")) (str.to_re ""))))) (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "4" (seq.++ "9" (seq.++ "3" ""))))(re.++ (re.union (re.range "\x00" "5") (re.range "7" "\xff"))(re.++ ((_ re.loop 12 12) (re.range "0" "9")) (str.to_re ""))))))))))))))
