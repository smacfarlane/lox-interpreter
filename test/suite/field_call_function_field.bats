# bats file_tags=tag:field
skip
@test "field/call_function_field.lox" {
  run target/debug/lox test/cases/field/call_function_field.lox

  [ "${lines[0]}" = "bar" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "2" ]
}
