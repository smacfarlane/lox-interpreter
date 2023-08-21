# bats file_tags=tag:variable
@test "variable/collide_with_parameter.lox" {
  run target/debug/lox test/cases/variable/collide_with_parameter.lox

}
