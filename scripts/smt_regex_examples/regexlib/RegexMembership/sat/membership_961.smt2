(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 0 3) (re.++ ((_ re.loop 1 6) (re.range "a" "z")) (re.union (re.range " " " ") (re.range "'" "'"))))(re.++ ((_ re.loop 0 3) (re.++ (re.union (re.range "A" "Z")(re.union (re.range "\xc4" "\xc5")(re.union (re.range "\xc7" "\xcb")(re.union (re.range "\xce" "\xcf")(re.union (re.range "\xd4" "\xd4")(re.union (re.range "\xd6" "\xd6") (re.range "\xdb" "\xdc")))))))(re.++ (re.++ ((_ re.loop 2 2) (re.union (re.range "a" "z")(re.union (re.range "\xe2" "\xe2")(re.union (re.range "\xe4" "\xe5")(re.union (re.range "\xe7" "\xeb")(re.union (re.range "\xee" "\xef")(re.union (re.range "\xf4" "\xf4")(re.union (re.range "\xf6" "\xf6") (re.range "\xfb" "\xfc"))))))))) (re.* (re.union (re.range "a" "z")(re.union (re.range "\xe2" "\xe2")(re.union (re.range "\xe4" "\xe5")(re.union (re.range "\xe7" "\xeb")(re.union (re.range "\xee" "\xef")(re.union (re.range "\xf4" "\xf4")(re.union (re.range "\xf6" "\xf6") (re.range "\xfb" "\xfc")))))))))) (re.union (re.range " " " ")(re.union (re.range "'" "'") (re.range "-" "-"))))))(re.++ (re.range "A" "Z")(re.++ (re.++ ((_ re.loop 2 2) (re.union (re.range "a" "z")(re.union (re.range "\xe2" "\xe2")(re.union (re.range "\xe4" "\xe5")(re.union (re.range "\xe7" "\xeb")(re.union (re.range "\xee" "\xef")(re.union (re.range "\xf4" "\xf4")(re.union (re.range "\xf6" "\xf6") (re.range "\xfb" "\xfc"))))))))) (re.* (re.union (re.range "a" "z")(re.union (re.range "\xe2" "\xe2")(re.union (re.range "\xe4" "\xe5")(re.union (re.range "\xe7" "\xeb")(re.union (re.range "\xee" "\xef")(re.union (re.range "\xf4" "\xf4")(re.union (re.range "\xf6" "\xf6") (re.range "\xfb" "\xfc")))))))))) (str.to_re "")))))))
