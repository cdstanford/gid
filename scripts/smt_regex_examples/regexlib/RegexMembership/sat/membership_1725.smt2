(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ (re.range "0" "1") (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "-" "/")(re.union (re.range "\x5c" "\x5c")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))))))(re.++ (re.union (re.++ (re.opt (re.range "(" "("))(re.++ (re.range "2" "9")(re.++ ((_ re.loop 2 2) (re.range "0" "9")) (re.opt (re.range ")" ")"))))) (re.++ (re.range "2" "9") ((_ re.loop 3 3) (re.range "0" "9"))))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "-" "/")(re.union (re.range "\x5c" "\x5c")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))))) (re.union (re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "-" "/")(re.union (re.range "\x5c" "\x5c")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))))) ((_ re.loop 4 4) (re.range "0" "9"))))(re.union ((_ re.loop 7 7) (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.range "-" "-") ((_ re.loop 4 4) (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))))))))))))
