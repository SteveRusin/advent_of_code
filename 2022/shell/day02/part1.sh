#! /usr/local/bin/bash

# Opponent
# A - Rock
# B - Paper
# C - Scissort

# Me
# X - Rock 1
# Y - Paper 2
# Z - Scissors 3

# 0 - lost
# 3 - draw
# 6 - won

# Rock -> Scissors
# Paper -> Rock
# Scissors -> Paper

# Result should be 15 for test

declare -A round_score_map=(
  ["X"]=1
  ["Y"]=2
  ["Z"]=3
);

declare -A win_map=(
  ["X"]="C"
  ["Y"]="A"
  ["Z"]="B"
);

declare -A opponent_to_my_choice_converter_map=(
  ["A"]="X"
  ["B"]="Y"
  ["C"]="Z"
)

my_score=0;
draw_score=3;
won_score=6;

while read line; do
  IFS=' ' read -r -a round <<< $line;
  opponent_choice="${round[0]}";
  my_choice=${round[1]};
  converted_opponent_choice="${opponent_to_my_choice_converter_map[$opponent_choice]}"

  my_score=$(($my_score + ${round_score_map[$my_choice]}));

  #draw
  if [ "$converted_opponent_choice" = "$my_choice" ]; then
    my_score=$(($my_score + $draw_score));
  #I won
  elif [ "${win_map[$my_choice]}" = "$opponent_choice" ]; then
    my_score=$(($my_score + $won_score));
  fi

done < input.txt

echo "${my_score}";
