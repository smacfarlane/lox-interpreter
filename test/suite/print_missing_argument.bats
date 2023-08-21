# bats file_tags=tag:print
skip
@test "print/missing_argument.lox" {
  run target/debug/lox test/cases/print/missing_argument.lox

}
