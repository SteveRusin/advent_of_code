#! /usr/local/bin/bash

# X - to lose
# Y - draw
# Z - need to win


declare -A round_score_map=(
  ["X"]=1
  ["Y"]=2
  ["Z"]=3
);


declare -A win_converter_map=(
  ["A"]="Y"
  ["B"]="Z"
  ["C"]="X"
);

declare -A draw_converter_map=(
  ["A"]="X"
  ["B"]="Y"
  ["C"]="Z"
);

declare -A lose_converter_map=(
  ["A"]="Z"
  ["B"]="X"
  ["C"]="Y"
);

declare -A opponent_to_my_choice_converter_map=(
  ["A"]="X"
  ["B"]="Y"
  ["C"]="Z"
);

declare -A win_map=(
  ["X"]="C"
  ["Y"]="A"
  ["Z"]="B"
);

#  ["X"]="lose_map"
#  ["Y"]="draw_map"
#  ["Z"]="win_map"

my_score=0;
draw_score=3;
won_score=6;

curr_dir=$(realpath "${BASH_SOURCE:-$0}" | xargs dirname)

while read line; do
  IFS=' ' read -r -a round <<< $line;
  opponent_choice="${round[0]}";
  my_choice=${round[1]};
  converted_opponent_choice="${opponent_to_my_choice_converter_map[$opponent_choice]}"
  converted_my_choice=""

  if [ "$my_choice" = "X" ]; then
    converted_my_choice="${lose_converter_map[$opponent_choice]}";
  elif [ "$my_choice" = "Y" ]; then
    converted_my_choice="${draw_converter_map[$opponent_choice]}";
  else
    converted_my_choice="${win_converter_map[$opponent_choice]}";
  fi

  round_score="${round_score_map[$converted_my_choice]}";
  my_score=$(($my_score + $round_score));

  echo "my current round score is: $round_score for chosing $converted_my_choice"

  echo "opponents converted choice: $converted_opponent_choice"

  #draw
  if [ "$converted_opponent_choice" = "$converted_my_choice" ]; then
    my_score=$(($my_score + $draw_score));
    echo "draw; My score now is: $my_score"
  #I won
  elif [ "${win_map[$converted_my_choice]}" = "$opponent_choice" ]; then
    my_score=$(($my_score + $won_score));
    echo "I won; My score now is: $my_score"
  else
    echo "I lost; My score now is: $my_score"
  fi

done < "$curr_dir/input.txt"

echo "${my_score}";

