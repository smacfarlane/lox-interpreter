# bats file_tags=tag:number
@test "number/literals.lox" {
  run target/debug/lox test/cases/number/literals.lox

  [ "${lines[0]}" = "123" ]
  [ "${lines[1]}" = "987654" ]
  [ "${lines[2]}" = "0" ]
  [ "${lines[3]}" = "-0" ]
  [ "${lines[4]}" = "123.456" ]
  [ "${lines[5]}" = "-0.001" ]
}
