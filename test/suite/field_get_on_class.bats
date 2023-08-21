# bats file_tags=tag:field
skip
@test "field/get_on_class.lox" {
  run target/debug/lox test/cases/field/get_on_class.lox

}
