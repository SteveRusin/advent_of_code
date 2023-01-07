#!/bin/bash 

divided_with_semicolon=$(awk -v RS= '{print ($0 ";")}' input.txt)
one_line=$(echo "${divided_with_semicolon}" | tr '\n' ' ')


IFS=";" read -ra ELFS <<< "$one_line"
elf_number=0;
elf_biggers_calories=0;
wlf_with_biggest_calories_number=1;
for ELF in "${ELFS[@]}"; do
  elf_number=$((elf_number + 1))
  while IFS=" " read -ra ONE_ELF_CALORIES; do
    ELF_CALORIES=0;
    for CALORIE in "${ONE_ELF_CALORIES[@]}"; do
      ELF_CALORIES=$(($ELF_CALORIES + $CALORIE));
      echo "${elf_number} ${CALORIE}: ${ELF_CALORIES}"
    done
      echo "elf_biggers_calories:${elf_biggers_calories}; ELF_CALORIES:${ELF_CALORIES}"
    if [ "$ELF_CALORIES" -gt "$elf_biggers_calories" ]; then
      elf_biggers_calories=$ELF_CALORIES;
      wlf_with_biggest_calories_number=$elf_number;
    fi
  done <<< "$ELF"
done
echo "max cal:${elf_biggers_calories}; elf num: ${wlf_with_biggest_calories_number}"
