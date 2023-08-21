# bats file_tags=tag:super
skip
@test "super/extra_arguments.lox" {
  run target/debug/lox test/cases/super/extra_arguments.lox

  [ "${lines[0]}" = "Derived.foo()" ]
}
