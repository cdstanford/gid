(explore-derivatives (re.++ (re.range "[" "[")(re.++ (re.* (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.++ (re.range "]" "]")(re.++ (re.+ (re.union (re.range "\x00" "Z") (re.range "\x5c" "\xff")))(re.++ (str.to_re (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ "-" (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " (seq.++ " " ""))))))))))))))))))))))))(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.+ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")))(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.range "=" "=")(re.++ (re.* (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ (re.+ (re.union (re.range "\x00" "\x0c") (re.range "\x0e" "\xff"))) (re.range " " " ")))))))))))))
