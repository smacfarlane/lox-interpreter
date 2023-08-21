# bats file_tags=tag:field
skip
@test "field/method.lox" {
  run target/debug/lox test/cases/field/method.lox

  [ "${lines[0]}" = "got method" ]
  [ "${lines[1]}" = "arg" ]
}
