(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ ((_ re.loop 20 20) (re.range "0" "9")) (str.to_re "")))(re.union (re.++ (str.to_re "")(re.++ (re.union ((_ re.loop 6 6) (re.++ (re.range ":" ":") ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))))) (str.to_re (seq.++ ":" (seq.++ ":" ""))))(re.++ (str.to_re (seq.++ "f" (seq.++ "f" (seq.++ "f" (seq.++ "f" (seq.++ ":" ""))))))(re.++ (re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "5" ""))) (re.range "0" "5"))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4") (re.range "0" "9")))(re.union (re.++ (re.range "1" "1")(re.++ (re.range "0" "9") (re.range "0" "9"))) ((_ re.loop 1 2) (re.range "0" "9")))))(re.++ ((_ re.loop 3 3) (re.++ (re.range "." ".") (re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "5" ""))) (re.range "0" "5"))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4") (re.range "0" "9")))(re.union (re.++ (re.range "1" "1")(re.++ (re.range "0" "9") (re.range "0" "9"))) ((_ re.loop 1 2) (re.range "0" "9"))))))) (str.to_re ""))))))(re.union (re.++ (str.to_re "")(re.++ (re.union ((_ re.loop 6 6) (re.++ (re.range ":" ":") ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))))) (str.to_re (seq.++ ":" (seq.++ ":" ""))))(re.++ (str.to_re (seq.++ "f" (seq.++ "f" (seq.++ "f" (seq.++ "f" "")))))(re.++ ((_ re.loop 2 2) (re.++ (re.range ":" ":") ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))))) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))(re.++ (re.range " " " ")(re.++ ((_ re.loop 7 7) (re.++ (re.range ":" ":") ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))))) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (re.range ":" ":")(re.++ ((_ re.loop 1 6) (re.++ (re.range ":" ":")(re.++ ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))) (re.opt (str.to_re (seq.++ ":" (seq.++ ":" ""))))))) (str.to_re ""))))(re.union (re.++ (str.to_re "")(re.++ ((_ re.loop 1 6) (re.++ (re.opt (str.to_re (seq.++ ":" (seq.++ ":" ""))))(re.++ ((_ re.loop 1 4) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))) (re.range ":" ":"))))(re.++ (re.range ":" ":") (str.to_re "")))) (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ ":" (seq.++ ":" ""))) (str.to_re ""))))))))))
