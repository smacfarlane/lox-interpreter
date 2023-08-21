# bats file_tags=tag:field
skip
@test "field/get_and_set_method.lox" {
  run target/debug/lox test/cases/field/get_and_set_method.lox

  [ "${lines[0]}" = "other" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "method" ]
  [ "${lines[3]}" = "2" ]
}
