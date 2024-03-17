#! /bin/sh
echo Hello ...

myvar="hello"

echo $myvar

for i in {1..10}; do
  if (( $i % 2 == 0 )); then
    echo "$i is even."
  else
    echo "$i is odd."
  fi
done

dirname="dir_$Test"
  if [ ! -d "$dirname" ]; then
    mkdir "$dirname"
    echo "Directory $dirname created."
  else
    echo "Directory $dirname already exists."
  fi