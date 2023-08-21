# bats file_tags=tag:scanning
skip
@test "scanning/numbers.lox" {
  run target/debug/lox test/cases/scanning/numbers.lox

  [ "${lines[0]}" = "NUMBER 123 123.0" ]
  [ "${lines[1]}" = "NUMBER 123.456 123.456" ]
  [ "${lines[2]}" = "DOT . null" ]
  [ "${lines[3]}" = "NUMBER 456 456.0" ]
  [ "${lines[4]}" = "NUMBER 123 123.0" ]
  [ "${lines[5]}" = "DOT . null" ]
  [ "${lines[6]}" = "EOF  null" ]
}
