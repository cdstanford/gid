(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ (re.union (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" "")))))(re.union (str.to_re (seq.++ "H" (seq.++ "T" (seq.++ "T" (seq.++ "P" "")))))(re.union (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" (seq.++ "s" ""))))))(re.union (str.to_re (seq.++ "H" (seq.++ "T" (seq.++ "T" (seq.++ "P" (seq.++ "S" ""))))))(re.union (str.to_re (seq.++ "f" (seq.++ "t" (seq.++ "p" "")))) (re.++ (str.to_re (seq.++ "F" (seq.++ "T" ""))) (re.opt (re.range "P" "P")))))))) (str.to_re (seq.++ ":" (seq.++ "/" (seq.++ "/" ""))))))(re.++ (re.+ (re.++ (re.+ (re.union (str.to_re (seq.++ "w" (seq.++ "w" (seq.++ "w" "")))) (str.to_re (seq.++ "W" (seq.++ "W" (seq.++ "W" "")))))) (re.range "." ".")))(re.++ (re.union (re.++ ((_ re.loop 3 3) ((_ re.loop 1 3) (re.range "0" "9")))(re.++ ((_ re.loop 1 3) (re.range "0" "9")) (re.range "." "."))) (re.++ (re.* (re.++ (re.+ (re.union (re.range "!" "!")(re.union (re.range "'" "*")(re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "~" "~")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))))))) (re.range "." ".")))(re.++ (re.opt (re.++ (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "^" "_")(re.union (re.range "a" "z")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))) ((_ re.loop 0 61) (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))))))(re.++ (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))(re.++ (re.range "." ".") ((_ re.loop 2 6) (re.range "a" "z")))))))(re.++ (re.opt (re.++ (re.range ":" ":") ((_ re.loop 1 4) (re.range "0" "9"))))(re.++ (re.union (re.* (re.range "/" "/")) (re.++ (re.+ (re.++ (re.+ (re.range "/" "/")) (re.+ (re.union (re.range "!" "!")(re.union (re.range "#" ".")(re.union (re.range "0" ";")(re.union (re.range "=" "=")(re.union (re.range "?" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "~" "~")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff"))))))))))))))))) (re.* (re.range "/" "/")))) (str.to_re ""))))))))
