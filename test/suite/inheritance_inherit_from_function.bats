# bats file_tags=tag:inheritance
skip
@test "inheritance/inherit_from_function.lox" {
  run target/debug/lox test/cases/inheritance/inherit_from_function.lox

}
