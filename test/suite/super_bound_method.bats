# bats file_tags=tag:super
skip
@test "super/bound_method.lox" {
  run target/debug/lox test/cases/super/bound_method.lox

  [ "${lines[0]}" = "A.method(arg)" ]
}
