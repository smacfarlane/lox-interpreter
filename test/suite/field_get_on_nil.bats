# bats file_tags=tag:field
skip
@test "field/get_on_nil.lox" {
  run target/debug/lox test/cases/field/get_on_nil.lox

}
