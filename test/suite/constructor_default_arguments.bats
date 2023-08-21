# bats file_tags=tag:constructor
skip
@test "constructor/default_arguments.lox" {
  run target/debug/lox test/cases/constructor/default_arguments.lox

}
