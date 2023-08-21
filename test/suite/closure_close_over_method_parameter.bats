# bats file_tags=tag:closure
skip
@test "closure/close_over_method_parameter.lox" {
  run target/debug/lox test/cases/closure/close_over_method_parameter.lox

  [ "${lines[0]}" = "param" ]
}
