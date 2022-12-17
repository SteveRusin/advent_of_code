#!/bin/bash 

divided_with_semicolon=$(awk -v RS= '{print ($0 ";")}' input.txt)
one_line=$(echo "${divided_with_semicolon}" | tr '\n' ' ')


IFS=";" read -ra ELFS <<< "$one_line"
ELF_NUMBER=0;
ELF_BIGGEST_CALORIES=0;
ELF_WITH_BIGGEST_CALORIES_NUMBER=1;
for ELF in "${ELFS[@]}"; do
  ELF_NUMBER=$((ELF_NUMBER + 1))
  while IFS=" " read -ra ONE_ELF_CALORIES; do
    ELF_CALORIES=0;
    for CALORIE in "${ONE_ELF_CALORIES[@]}"; do
      ELF_CALORIES=$(($ELF_CALORIES + $CALORIE));
      echo "${ELF_NUMBER} ${CALORIE}: ${ELF_CALORIES}"
    done
      echo "ELF_BIGGEST_CALORIES:${ELF_BIGGEST_CALORIES}; ELF_CALORIES:${ELF_CALORIES}"
    if [ "$ELF_CALORIES" -gt "$ELF_BIGGEST_CALORIES" ]; then
      ELF_BIGGEST_CALORIES=$ELF_CALORIES;
      ELF_WITH_BIGGEST_CALORIES_NUMBER=$ELF_NUMBER;
    fi
  done <<< "$ELF"
done
echo "max cal:${ELF_BIGGEST_CALORIES}; elf num: ${ELF_WITH_BIGGEST_CALORIES_NUMBER}"
