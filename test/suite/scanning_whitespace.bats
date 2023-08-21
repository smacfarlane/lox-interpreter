# bats file_tags=tag:scanning
skip
@test "scanning/whitespace.lox" {
  run target/debug/lox test/cases/scanning/whitespace.lox

  [ "${lines[0]}" = "IDENTIFIER space null" ]
  [ "${lines[1]}" = "IDENTIFIER tabs null" ]
  [ "${lines[2]}" = "IDENTIFIER newlines null" ]
  [ "${lines[3]}" = "IDENTIFIER end null" ]
  [ "${lines[4]}" = "EOF  null" ]
}
