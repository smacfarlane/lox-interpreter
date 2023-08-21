# bats file_tags=tag:method
skip
@test "method/print_bound_method.lox" {
  run target/debug/lox test/cases/method/print_bound_method.lox

  [ "${lines[0]}" = "<fn method>" ]
}
