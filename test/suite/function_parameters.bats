# bats file_tags=tag:function
skip
@test "function/parameters.lox" {
  run target/debug/lox test/cases/function/parameters.lox

  [ "${lines[0]}" = "0" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "3" ]
  [ "${lines[3]}" = "6" ]
  [ "${lines[4]}" = "10" ]
  [ "${lines[5]}" = "15" ]
  [ "${lines[6]}" = "21" ]
  [ "${lines[7]}" = "28" ]
  [ "${lines[8]}" = "36" ]
}
