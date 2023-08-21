# bats file_tags=tag:scanning
skip
@test "scanning/strings.lox" {
  run target/debug/lox test/cases/scanning/strings.lox

  [ "${lines[0]}" = "STRING """ ]
  [ "${lines[1]}" = "STRING "string" string" ]
  [ "${lines[2]}" = "EOF  null" ]
}
