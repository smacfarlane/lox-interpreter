# bats file_tags=tag:number
skip
@test "number/trailing_dot.lox" {
  run target/debug/lox test/cases/number/trailing_dot.lox

}
