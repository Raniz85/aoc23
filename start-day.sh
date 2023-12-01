#!/bin/bash
if [[ "$#" != "1" ]];
then
  echo "Usage: $0 DAY"
  echo "Copies the template to a new day and downloads the puzzle input"
  echo "Put the value of the 'session' cookie for 'adventofcode.com' in .env like this:"
  echo
  echo 'AOC_SESSION="<your session cookie here>'
  exit 1
fi
source .env
day=$1
prefixed=$(printf '%02d' $day)
target="day${prefixed}"
cp -r template day${prefixed}
sed -i "s/dayXX/day${prefixed}/" ${target}/Cargo.toml ${target}/src/main.rs

head -n -1 Cargo.toml >Cargo.toml.new
echo "  \"day${prefixed}\"," >>Cargo.toml.new
echo "]" >>Cargo.toml.new
mv Cargo.toml.new Cargo.toml

exec curl "https://adventofcode.com/2022/day/${1}/input" -H "Cookie: session=${AOC_SESSION}" -o "day${prefixed}/input"
