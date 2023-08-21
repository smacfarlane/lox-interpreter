# bats file_tags=tag:field
skip
@test "field/undefined.lox" {
  run target/debug/lox test/cases/field/undefined.lox

}
