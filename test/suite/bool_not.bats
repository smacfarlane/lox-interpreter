# bats file_tags=tag:bool
skip
@test "bool/not.lox" {
  run target/debug/lox test/cases/bool/not.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "true" ]
  [ "${lines[2]}" = "true" ]
}
