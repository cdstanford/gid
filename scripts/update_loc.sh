find . -name '*.rs' -not -path "./target/*" | xargs wc -l | sort > loc.txt
