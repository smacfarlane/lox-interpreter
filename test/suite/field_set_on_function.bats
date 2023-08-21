# bats file_tags=tag:field
skip
@test "field/set_on_function.lox" {
  run target/debug/lox test/cases/field/set_on_function.lox

}
