# bats file_tags=tag:string
@test "string/literals.lox" {
  run target/debug/lox test/cases/string/literals.lox

  [ "${lines[0]}" = "()" ]
  [ "${lines[1]}" = "a string" ]
  [ "${lines[2]}" = "A~¶Þॐஃ" ]
}
