# bats file_tags=tag:number
skip
@test "number/leading_dot.lox" {
  run target/debug/lox test/cases/number/leading_dot.lox

}
