# bats file_tags=tag:constructor
skip
@test "constructor/missing_arguments.lox" {
  run target/debug/lox test/cases/constructor/missing_arguments.lox

}
