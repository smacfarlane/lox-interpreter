# bats file_tags=tag:if
@test "if/truth.lox" {
  run target/debug/lox test/cases/if/truth.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "nil" ]
  [ "${lines[2]}" = "true" ]
  [ "${lines[3]}" = "0" ]
  [ "${lines[4]}" = "empty" ]
}
