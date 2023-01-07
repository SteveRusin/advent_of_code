#!/bin/bash 

divided_with_semicolon=$(awk -v RS= '{print ($0 ";")}' input.txt)
one_line=$(echo "${divided_with_semicolon}" | tr '\n' ' ')


IFS=";" read -ra ELFS <<< "$one_line"
total_calories_by_elf=();
for ELF in "${ELFS[@]}"; do
  ELF_NUMBER=$((ELF_NUMBER + 1))
  while IFS=" " read -ra one_elf_calorie; do
    elf_calories=0;
    for calorie in "${one_elf_calorie[@]}"; do
      elf_calories=$(($elf_calories + $calorie));
      #echo "${ELF_NUMBER} ${calorie}: ${elf_calories}"
    done
    if [ "$elf_calories" -gt 0 ]; then
      total_calories_by_elf+=("${elf_calories}");
    fi
  done <<< "$ELF"
done

IFS=$'\n' SORTED=($(sort -n -r <<< "${total_calories_by_elf[*]}"))

declare -a top_three_calories=("${SORTED[*]:0:3}");

for calorie in $top_three_calories
do
  top_calories_by_three_elfs=$(($top_calories_by_three_elfs + $calorie));
done
echo "${top_calories_by_three_elfs}"
