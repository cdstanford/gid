(explore-derivatives (re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.union (str.to_re (seq.++ "i" (seq.++ "n" (seq.++ "t" ""))))(re.union (str.to_re (seq.++ "v" (seq.++ "o" (seq.++ "i" (seq.++ "d" "")))))(re.union (str.to_re (seq.++ "f" (seq.++ "l" (seq.++ "o" (seq.++ "a" (seq.++ "t" ""))))))(re.union (str.to_re (seq.++ "c" (seq.++ "h" (seq.++ "a" (seq.++ "r" "")))))(re.union (str.to_re (seq.++ "d" (seq.++ "o" (seq.++ "u" (seq.++ "b" (seq.++ "l" (seq.++ "e" ""))))))) (str.to_re (seq.++ "s" (seq.++ "t" (seq.++ "r" (seq.++ "i" (seq.++ "n" (seq.++ "g" ""))))))))))))(re.++ (re.* (re.union (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) (re.range "*" "*")))(re.++ (re.opt (re.range "&" "&"))(re.++ (re.+ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "a" "z")(re.++ (re.* (re.union (re.range "0" "9") (re.range "a" "z")))(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "(" "(")(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.opt (re.++ (re.union (str.to_re (seq.++ "i" (seq.++ "n" (seq.++ "t" ""))))(re.union (str.to_re (seq.++ "v" (seq.++ "o" (seq.++ "i" (seq.++ "d" "")))))(re.union (str.to_re (seq.++ "f" (seq.++ "l" (seq.++ "o" (seq.++ "a" (seq.++ "t" ""))))))(re.union (str.to_re (seq.++ "c" (seq.++ "h" (seq.++ "a" (seq.++ "r" "")))))(re.union (str.to_re (seq.++ "d" (seq.++ "o" (seq.++ "u" (seq.++ "b" (seq.++ "l" (seq.++ "e" ""))))))) (str.to_re (seq.++ "s" (seq.++ "t" (seq.++ "r" (seq.++ "i" (seq.++ "n" (seq.++ "g" ""))))))))))))(re.++ (re.* (re.union (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) (re.range "*" "*")))(re.++ (re.opt (re.range "&" "&"))(re.++ (re.+ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "a" "z")(re.++ (re.* (re.union (re.range "0" "9") (re.range "a" "z"))) (re.* (re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "," ",")(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.union (str.to_re (seq.++ "i" (seq.++ "n" (seq.++ "t" ""))))(re.union (str.to_re (seq.++ "v" (seq.++ "o" (seq.++ "i" (seq.++ "d" "")))))(re.union (str.to_re (seq.++ "f" (seq.++ "l" (seq.++ "o" (seq.++ "a" (seq.++ "t" ""))))))(re.union (str.to_re (seq.++ "c" (seq.++ "h" (seq.++ "a" (seq.++ "r" "")))))(re.union (str.to_re (seq.++ "d" (seq.++ "o" (seq.++ "u" (seq.++ "b" (seq.++ "l" (seq.++ "e" ""))))))) (str.to_re (seq.++ "s" (seq.++ "t" (seq.++ "r" (seq.++ "i" (seq.++ "n" (seq.++ "g" ""))))))))))))(re.++ (re.* (re.union (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) (re.range "*" "*")))(re.++ (re.opt (re.range "&" "&"))(re.++ (re.+ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "a" "z") (re.* (re.union (re.range "0" "9") (re.range "a" "z")))))))))))))))))))(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range ")" ")")(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))) (re.range ";" ";"))))))))))))))))
