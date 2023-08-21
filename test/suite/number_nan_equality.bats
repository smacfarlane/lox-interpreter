# bats file_tags=tag:number
@test "number/nan_equality.lox" {
  run target/debug/lox test/cases/number/nan_equality.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "true" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
}
