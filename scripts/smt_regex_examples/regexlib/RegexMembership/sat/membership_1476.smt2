(explore-derivatives (re.union (re.++ (str.to_re "") (re.++ ((_ re.loop 3 3) (re.range "A" "Z"))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.union ((_ re.loop 3 3) (re.range "0" "9"))(re.union ((_ re.loop 2 2) (re.range "0" "9")) (re.range "d" "d")))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))) (re.range "A" "Z"))))))(re.union (re.++ (re.range "A" "Z")(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.union ((_ re.loop 3 3) (re.range "0" "9"))(re.union ((_ re.loop 2 2) (re.range "0" "9")) (re.range "0" "9")))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))) ((_ re.loop 3 3) (re.range "A" "Z")))))) (re.++ (re.++ (re.++ (re.union (re.range "A" "H")(re.union (re.range "K" "P")(re.union (re.range "R" "S")(re.union (re.range "V" "W") (re.range "Y" "Y"))))) (re.union (re.range "A" "H")(re.union (re.range "J" "P") (re.range "R" "Y"))))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.union (re.++ (re.range "0" "0") (re.range "2" "9")) (re.++ (re.range "1" "9") (re.range "0" "9")))(re.++ (re.opt (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))) ((_ re.loop 3 3) (re.union (re.range "A" "H")(re.union (re.range "J" "P") (re.range "R" "Z")))))))) (str.to_re "")))))
