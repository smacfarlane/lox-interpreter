# bats file_tags=tag:field
skip
@test "field/get_on_string.lox" {
  run target/debug/lox test/cases/field/get_on_string.lox

}
