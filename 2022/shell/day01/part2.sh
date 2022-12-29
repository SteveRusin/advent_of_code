#!/bin/bash 

divided_with_semicolon=$(awk -v RS= '{print ($0 ";")}' input.txt)
one_line=$(echo "${divided_with_semicolon}" | tr '\n' ' ')


IFS=";" read -ra ELFS <<< "$one_line"
TOTAL_CALORIES_BY_ELFS=();
for ELF in "${ELFS[@]}"; do
  ELF_NUMBER=$((ELF_NUMBER + 1))
  while IFS=" " read -ra ONE_ELF_CALORIES; do
    ELF_CALORIES=0;
    for CALORIE in "${ONE_ELF_CALORIES[@]}"; do
      ELF_CALORIES=$(($ELF_CALORIES + $CALORIE));
      #echo "${ELF_NUMBER} ${CALORIE}: ${ELF_CALORIES}"
    done
    if [ "$ELF_CALORIES" -gt 0 ]; then
      TOTAL_CALORIES_BY_ELFS+=("${ELF_CALORIES}");
    fi
  done <<< "$ELF"
done

IFS=$'\n' SORTED=($(sort -n -r <<< "${TOTAL_CALORIES_BY_ELFS[*]}"))

declare -a TOP_THREE_CALORIES=("${SORTED[*]:0:3}");

for CALORIE in $TOP_THREE_CALORIES
do
  TOP_CALORIES_BY_THREE_ELFS=$(($TOP_CALORIES_BY_THREE_ELFS + $CALORIE));
done
echo "${TOP_CALORIES_BY_THREE_ELFS}"
